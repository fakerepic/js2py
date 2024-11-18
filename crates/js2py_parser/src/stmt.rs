use syntax::lex::TokenTypeUtil;

use super::*;
use crate::ast::*;

impl<'a> Parser<'a> {
    pub(crate) fn parse_statements(&mut self, is_top_level: bool) -> Result<Vec<Statement<'a>>> {
        let mut statements = vec![];
        while !self.at(Type::EOF) {
            if !is_top_level && self.at(Type::RCurly) {
                break;
            }
            statements.push(self.parse_statement(StatementContext::StatementList)?);
        }

        Ok(statements)
    }
    pub(crate) fn parse_statement(&mut self, stmt_ctx: StatementContext) -> Result<Statement<'a>> {
        match self.cur_kind() {
            Type::LCurly => self.parse_block_statement(),
            Type::Semicolon => Ok(self.parse_empty_statement()),
            Type::If => self.parse_if_statement(),
            Type::Do => self.parse_do_while_statement(),
            Type::While => self.parse_while_statement(),
            Type::For => self.parse_for_statement(),
            Type::Break | Type::Continue => self.parse_break_or_continue_statement(),
            Type::Switch => self.parse_switch_statement(),
            Type::Return => self.parse_return_statement(),
            Type::Function => self.parse_function_declaration(stmt_ctx),
            kind if kind.is_variable_declaration() => self.parse_variable_statement(stmt_ctx),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_break_or_continue_statement(&mut self) -> Result<Statement<'a>> {
        let start_span = self.start_span();
        let kind = self.cur_kind();
        self.bump_any();
        self.auto_semicoclon_insertion()?;
        let span = self.end_span(start_span);
        match kind {
            Type::Break => Ok(Statement::BreakStatement(Box::new(BreakStatement { span }))),
            Type::Continue => Ok(Statement::ContinueStatement(Box::new(ContinueStatement {
                span,
            }))),
            _ => unreachable!(),
        }
    }

    fn parse_do_while_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.bump_any(); // advance `do`
        let body = self.parse_statement(StatementContext::Do)?;
        self.expect(Type::While)?;
        let test = self.parse_paren_expression()?;
        self.bump(Type::Semicolon);
        Ok(Statement::DoWhileStatement(Box::new(DoWhileStatement {
            span: self.end_span(span),
            body,
            test,
        })))
    }

    fn parse_while_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.bump_any(); // bump `while`
        let test = self.parse_paren_expression()?;
        let body = self.parse_statement(StatementContext::While)?;
        Ok(Statement::WhileStatement(Box::new(WhileStatement {
            span: self.end_span(span),
            test,
            body,
        })))
    }

    fn parse_for_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.bump_any(); // bump `for`

        self.expect(Type::LParen)?;

        // for (;..
        if self.at(Type::Semicolon) {
            return self.parse_for_loop(span, None);
        }

        // for (let | for (const | for (var
        if self.cur_kind().is_variable_declaration() {
            return self.parse_variable_declaration_for_statement(span);
        }

        if self.at(Type::RParen) {
            return self.parse_for_loop(span, None);
        }

        let init_expression = self.parse_expr()?;

        self.parse_for_loop(span, Some(ForStatementInit::Expression(init_expression)))
    }

    fn parse_for_loop(
        &mut self,
        span: Span,
        init: Option<ForStatementInit<'a>>,
    ) -> Result<Statement<'a>> {
        self.expect(Type::Semicolon)?;
        let test = if !self.at(Type::Semicolon) && !self.at(Type::RParen) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(Type::Semicolon)?;
        let update = if self.at(Type::RParen) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(Type::RParen)?;
        let body = self.parse_statement(StatementContext::For)?;
        Ok(Statement::ForStatement(Box::new(ForStatement {
            span: self.end_span(span),
            init,
            test,
            update,
            body,
        })))
    }

    fn parse_variable_declaration_for_statement(&mut self, span: Span) -> Result<Statement<'a>> {
        let start_span = self.start_span();
        let init_declaration = self.parse_variable_declaration(start_span)?;
        let init = Some(ForStatementInit::VariableDeclaration(Box::new(
            init_declaration,
        )));
        self.parse_for_loop(span, init)
    }

    fn parse_return_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.bump_any(); // advance `return`
        let argument = if self.eat(Type::Semicolon) || self.can_insert_semicolon() {
            None
        } else {
            // TODO: `In` context
            // let expr = self.context(Context::In, Context::empty(), ParserImpl::parse_expr)?;
            let expr = self.parse_expr()?;
            self.auto_semicoclon_insertion()?;
            Some(expr)
        };
        Ok(Statement::ReturnStatement(Box::new(ReturnStatement {
            span: self.end_span(span),
            argument,
        })))
    }

    fn parse_empty_statement(&mut self) -> Statement<'a> {
        let span = self.start_span();
        self.bump_any(); // bump `;`
        Statement::EmptyStatement(Box::new(EmptyStatement {
            span: self.end_span(span),
        }))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement<'a>> {
        let start_span = self.start_span();
        let expr = self.parse_expr()?;
        self.auto_semicoclon_insertion()?;
        Ok(Statement::ExpressionStatement(Box::new(
            ExpressionStatement {
                span: self.end_span(start_span),
                expression: expr,
            },
        )))
    }

    fn parse_block_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.expect(Type::LCurly)?;
        let mut body = vec![];
        while !self.at(Type::RCurly) && !self.at(Type::EOF) {
            let stmt = self.parse_statement(StatementContext::StatementList)?;
            body.push(stmt);
        }
        self.expect(Type::RCurly)?;

        Ok(Statement::BlockStatement(Box::new(BlockStatement {
            span: self.end_span(span),
            body,
        })))
    }

    fn parse_if_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.bump_any(); // bump `if`
        let test = self.parse_paren_expression()?;
        let consequent = self.parse_statement(StatementContext::If)?;
        let alternate = self
            .eat(Type::Else)
            .then(|| self.parse_statement(StatementContext::If))
            .transpose()?;

        Ok(Statement::IfStatement(Box::new(IfStatement {
            span: self.end_span(span),
            test,
            consequent,
            alternate,
        })))
    }

    fn parse_switch_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.bump_any(); // advance `switch`
        let discriminant = self.parse_paren_expression()?;
        let cases = self.parse_normal_list(Type::LCurly, Type::RCurly, Self::parse_switch_case)?;
        Ok(Statement::SwitchStatement(Box::new(SwitchStatement {
            span: self.end_span(span),
            discriminant,
            cases,
        })))
    }

    pub(crate) fn parse_switch_case(&mut self) -> Result<Option<SwitchCase<'a>>> {
        let span = self.start_span();
        let test = match self.cur_kind() {
            Type::Default => {
                self.bump_any();
                None
            }
            Type::Case => {
                self.bump_any();
                let expression = self.parse_expr()?;
                Some(expression)
            }
            _ => return Err(self.unexpected()),
        };
        self.expect(Type::Colon)?;
        let mut consequent = vec![];
        while !matches!(
            self.cur_kind(),
            Type::Case | Type::Default | Type::RCurly | Type::EOF
        ) {
            let stmt = self.parse_statement(StatementContext::StatementList)?;
            consequent.push(stmt);
        }
        Ok(Some(SwitchCase {
            span: self.end_span(span),
            test,
            consequent,
        }))
    }

    pub(crate) fn parse_normal_list<F, T>(
        &mut self,
        open: Type,
        close: Type,
        cb: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(&mut Self) -> Result<Option<T>>,
    {
        self.expect(open)?;
        let mut list = vec![];
        loop {
            let kind = self.cur_kind();
            if kind == close || kind == Type::EOF {
                break;
            }
            if let Some(e) = cb(self)? {
                list.push(e);
            } else {
                break;
            }
        }
        self.expect(close)?;
        Ok(list)
    }

    fn parse_variable_declaration(&mut self, start_span: Span) -> Result<VariableDeclaration<'a>> {
        let kind = match self.cur_kind() {
            Type::Var => VariableDeclarationKind::Var,
            Type::Let => VariableDeclarationKind::Let,
            Type::Const => VariableDeclarationKind::Const,
            _ => return Err(self.unexpected()),
        };
        self.bump_any();
        // self.bump(Type::Var);

        let id = self.parse_identifier()?;

        let init = self
            .eat(Type::Eq)
            .then(|| self.parse_assignment_expression_or_higher())
            .transpose()?;

        Ok(VariableDeclaration {
            span: self.end_span(start_span),
            kind,
            id,
            init,
        })
    }

    #[allow(unused_variables)]
    fn parse_variable_statement(&mut self, stmt_ctx: StatementContext) -> Result<Statement<'a>> {
        let start_span = self.start_span();
        let decl = self.parse_variable_declaration(start_span)?;
        Ok(Statement::VariableDeclarationStatement(Box::new(decl)))
    }
}
