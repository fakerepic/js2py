use super::*;
use crate::ast::*;
use std::sync::mpsc;

use js2py_lexer::lexer::token_stream;
use syntax::lex::TokenTypeUtil;

pub struct Parser<'a> {
    /// Source Code
    source: &'a str,

    lexer: mpsc::Receiver<Token>,

    /// Current Token consumed from the lexer
    cur_token: Token,

    /// The end range of the previous token
    prev_token_end: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: token_stream(source),
            cur_token: Token::default(),
            prev_token_end: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        self.parse_program()
    }

    pub fn parse_expression(mut self) -> Result<Expression<'a>> {
        // initialize cur_token and prev_token by moving onto the first token
        self.bump_any();
        let expr = self.parse_expr()?;
        Ok(expr)
    }

    fn parse_program(&mut self) -> Result<Program<'a>> {
        // initialize cur_token and prev_token by moving onto the first token
        self.bump_any();
        let start = self.start_span();
        let body = self.parse_statements(true)?;
        let end = self.prev_token_end;
        Ok(Program {
            span: Span::new(start.start, end),
            body,
            source_text: self.source,
        })
    }
}

// helpers:
impl<'a> Parser<'a> {
    pub(crate) fn start_span(&self) -> Span {
        let token = self.cur_token();
        Span::new(token.start, 0)
    }

    pub(crate) fn end_span(&self, mut span: Span) -> Span {
        span.end = self.prev_token_end;
        debug_assert!(span.end >= span.start);
        span
    }

    pub(crate) fn cur_token(&self) -> &Token {
        &self.cur_token
    }

    pub(crate) fn cur_kind(&self) -> Type {
        self.cur_token.typ
    }

    pub(crate) fn cur_string(&self) -> &'a str {
        self.source
            .get(self.cur_token.start..self.cur_token.end)
            .unwrap_or_default()
    }

    /// Checks if the current index has token `Type`
    pub(crate) fn at(&self, kind: Type) -> bool {
        self.cur_kind() == kind
    }

    /// Advance if we are at `Type`
    pub(crate) fn bump(&mut self, kind: Type) {
        if self.at(kind) {
            self.advance();
        }
    }

    /// Advance any token
    pub(crate) fn bump_any(&mut self) {
        self.advance();
    }

    /// Advance and return true if we are at `Type`, return false otherwise
    pub(crate) fn eat(&mut self, kind: Type) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    /// Move to the next token
    pub(crate) fn advance(&mut self) {
        let mut token = Token::default();
        while let Ok(t) = self.lexer.recv() {
            if matches!(t.typ, Type::LineTerminator) {
                continue;
            } else {
                token = t;
                break;
            }
        }
        self.prev_token_end = self.cur_token.end;
        self.cur_token = token;
    }

    pub(crate) fn expect_peek_only(&mut self, kind: Type) -> Result<()> {
        if !self.at(kind) {
            return Err(format!(
                "Expected token type {:?} but got {:?}",
                kind, self.cur_token
            ));
        }
        Ok(())
    }

    pub(crate) fn expect(&mut self, kind: Type) -> Result<()> {
        self.expect_peek_only(kind)?;
        self.advance();
        Ok(())
    }

    pub(crate) fn unexpected(&self) -> String {
        format!("Unexpected token: {:?}", self.cur_token)
    }

    pub(crate) fn can_insert_semicolon(&self) -> bool {
        let kind = self.cur_kind();
        if kind == Type::Semicolon {
            return true;
        }
        // TODO: check if cur_token is on a new line
        kind == Type::RCurly || kind.is_eof()
        // || self.cur_token().is_on_new_line
    }

    pub(crate) fn auto_semicoclon_insertion(&mut self) -> Result<()> {
        if !self.can_insert_semicolon() {
            let span = Span::new(self.prev_token_end, self.prev_token_end);
            return Err(format!(
                "Expected a semicolon or an implicit semicolon after a statement, but found none at {:?}",
                span
            ));
        }
        if self.at(Type::Semicolon) {
            self.advance();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;
    #[test]
    fn parse_empty_smoke_test() {
        let source = "";
        let mut parser = Parser::new(source);
        let ret = parser.parse();
        assert!(ret.is_ok());
        let r = ret.unwrap();
        assert!(r.body.is_empty());
    }

    #[test]
    fn parse_program_smoke_test() {
        let source = "a;\nb;";
        let mut parser = Parser::new(source);
        let ret = parser.parse();
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(r.body.len() == 2);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_identifier_expression_test() {
        let source = "a";
        let mut parser = Parser::new(source);
        parser.bump_any();
        let ret = parser.parse_identifier_expression();
        match ret {
            Ok(r) => {
                assert!(matches!(r, Expression::Identifier(_)));
                println!("{:#?}", r);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_variable_declaration_statement_test() {
        let source = "var a = 11.1;";
        let mut parser = Parser::new(source);
        parser.bump_any();
        let ret = parser.parse_statement(StatementContext::StatementList);
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Statement::VariableDeclarationStatement(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_if_statement_test() {
        let source = "if (a) { c = a } else { return 1 }";
        let mut parser = Parser::new(source);
        parser.bump_any();
        let ret = parser.parse_statement(StatementContext::StatementList);
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Statement::IfStatement(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_for_statement_test() {
        let source = "for (let i = 0; i < 9; i += 1) { break }";
        let mut parser = Parser::new(source);
        parser.bump_any();
        let ret = parser.parse_statement(StatementContext::StatementList);
        match ret {
            Ok(r) => {
                assert!(matches!(r, Statement::ForStatement(_)));
                println!("{:#?}", r);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_switch_statement_test() {
        let source = "switch (a) { case 1: break; default: return 1 }";
        let mut parser = Parser::new(source);
        parser.bump_any();
        let ret = parser.parse_statement(StatementContext::StatementList);
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Statement::SwitchStatement(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_function_declaration_test() {
        let source = "function a() { return 1 }";
        let mut parser = Parser::new(source);
        parser.bump_any();
        let ret = parser.parse_statement(StatementContext::StatementList);
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Statement::FunctionDeclaration(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_array_expression_test() {
        let source = "[ 1, 'asdf', , 3 ]";
        let parser = Parser::new(source);
        let ret = parser.parse_expression();
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Expression::ArrayExpression(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_object_expression_test() {
        let source = "{ a: 1, b: { 'c' : 1 } }";
        let parser = Parser::new(source);
        let ret = parser.parse_expression();
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Expression::ObjectExpression(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_static_member_expression_test() {
        let source = "a.b.c";
        let parser = Parser::new(source);
        let ret = parser.parse_expression();
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Expression::StaticMemberExpression(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_computed_member_expression_test() {
        let source = "a[b]";
        let parser = Parser::new(source);
        let ret = parser.parse_expression();
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Expression::ComputedMemberExpression(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn parse_call_expression_test() {
        let source = "a(1,2)";
        let parser = Parser::new(source);
        let ret = parser.parse_expression();
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
                assert!(matches!(r, Expression::CallExpression(_)));
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }

    #[test]
    fn pratt_test() {
        let source = "(a + b) + c * c";
        let parser = Parser::new(source);
        let ret = parser.parse_expression();
        match ret {
            Ok(r) => {
                println!("{:#?}", r);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!()
            }
        }
    }
}
