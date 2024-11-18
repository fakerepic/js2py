use super::*;
use ast::*;
use syntax::lex::TokenTypeUtil;

impl<'a> Parser<'a> {
    #[allow(unused_variables)]
    pub(crate) fn parse_function_declaration(
        &mut self,
        stmt_ctx: StatementContext,
    ) -> Result<Statement<'a>> {
        let decl = self.parse_function_impl()?;

        Ok(Statement::FunctionDeclaration(decl))
    }

    pub(crate) fn parse_function_impl(&mut self) -> Result<Box<Function<'a>>> {
        let span = self.start_span();
        self.expect(Type::Function)?;
        let id = self.parse_function_id()?;
        self.parse_function(span, id)
    }

    pub(crate) fn parse_function_id(&mut self) -> Result<Option<Identifier<'a>>> {
        let id = self.cur_kind().is_identifier().then(|| {
            let (span, name) = self.parse_identifier_kind();
            Identifier { span, name }
        });

        Ok(id)
    }

    pub(crate) fn parse_formal_parameters(&mut self) -> Result<Box<FormalParameters<'a>>> {
        let span = self.start_span();
        self.expect(Type::LParen)?;

        let mut params = vec![];
        while !self.at(Type::RParen) {
            let (span, name) = self.parse_identifier_kind();
            params.push(Identifier { span, name });
            if self.at(Type::Comma) {
                self.bump_any();
            }
        }
        self.expect(Type::RParen)?;
        Ok(Box::new(FormalParameters { span, params }))
    }

    pub(crate) fn parse_function(
        &mut self,
        span: Span,
        id: Option<Identifier<'a>>,
    ) -> Result<Box<Function<'a>>> {
        let params = self.parse_formal_parameters()?;

        let body = if self.at(Type::LCurly) {
            Some(self.parse_function_body()?)
        } else {
            None
        };

        Ok(Box::new(Function {
            span: self.end_span(span),
            id,
            params,
            body,
        }))
    }

    pub(crate) fn parse_function_body(&mut self) -> Result<Box<FunctionBody<'a>>> {
        let span = self.start_span();
        self.expect(Type::LCurly)?;

        // TODO: consider context
        // let (directives, statements) = self.context(Context::Return, Context::empty(), |p| {
        //     p.parse_directives_and_statements(/* is_top_level */ false)
        // })?;
        let statements = self.parse_statements(false)?;

        self.expect(Type::RCurly)?;
        Ok(Box::new(FunctionBody {
            span: self.end_span(span),
            statements,
        }))
    }
}
