use op::*;
use syntax::lex::TokenTypeUtil;
use syntax::precedence::Precedence;

use super::*;
use ast::*;

#[allow(dead_code)]

impl<'a> Parser<'a> {
    pub(crate) fn parse_expr(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let lhs = self.parse_assignment_expression_or_higher()?;
        if !self.at(Type::Comma) {
            return Ok(lhs);
        }

        let expr = self.parse_sequence_expression(span, lhs)?;

        Ok(expr)
    }

    pub(crate) fn parse_paren_expression(&mut self) -> Result<Expression<'a>> {
        self.expect(Type::LParen)?;
        let expression = self.parse_expr()?;
        self.expect(Type::RParen)?;
        Ok(expression)
    }

    fn parse_sequence_expression(
        &mut self,
        span: Span,
        first_expression: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut expressions = vec![first_expression];
        while self.eat(Type::Comma) {
            let expression = self.parse_assignment_expression_or_higher()?;
            expressions.push(expression);
        }
        Ok(Expression::SequenceExpression(Box::new(
            SequenceExpression {
                span: self.end_span(span),
                expressions,
            },
        )))
    }

    pub(crate) fn parse_assignment_expression_or_higher(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let lhs = self.parse_binary_expression_or_higher(Precedence::Comma)?;
        let kind = self.cur_kind();
        if kind.is_assignment_operator() {
            return self.parse_assignment_expression_recursive(span, lhs);
        }

        Ok(lhs)
    }

    fn parse_assignment_expression_recursive(
        &mut self,
        span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let operator = map_assignment_operator(self.cur_kind());
        let left = match lhs {
            Expression::Identifier(ident) => AssignmentTarget::Identifier(ident),
            Expression::StaticMemberExpression(member) => {
                AssignmentTarget::StaticMemberExpression(member)
            }
            Expression::ComputedMemberExpression(member) => {
                AssignmentTarget::ComputedMemberExpression(member)
            }
            _ => return Err(format!("Invalid assignment target: {:?}", lhs)),
        };
        self.bump_any();
        let right = self.parse_assignment_expression_or_higher()?;
        Ok(Expression::AssignmentExpression(Box::new(
            AssignmentExpression {
                span: self.end_span(span),
                operator,
                left,
                right,
            },
        )))
    }

    pub(crate) fn parse_unary_expression_or_higher(
        &mut self,
        lhs_span: Span,
    ) -> Result<Expression<'a>> {
        let is_update_expression = !matches!(self.cur_kind(), kind if kind.is_unary_operator());

        if is_update_expression {
            return self.parse_update_expression(lhs_span);
        }
        self.parse_simple_unary_expression(lhs_span)
    }

    pub(crate) fn parse_simple_unary_expression(
        &mut self,
        lhs_span: Span,
    ) -> Result<Expression<'a>> {
        match self.cur_kind() {
            kind if kind.is_unary_operator() => self.parse_unary_expression(),
            _ => self.parse_update_expression(lhs_span),
        }
    }

    /// Section 13.4 Update Expression
    #[allow(unused_variables)]
    fn parse_update_expression(&mut self, lhs_span: Span) -> Result<Expression<'a>> {
        // TODO: support prefix update expressions

        // let kind = self.cur_kind();
        // // ++ -- prefix update expressions
        // if kind.is_update_operator() {
        //     ...
        // }

        // let span = self.start_span();
        let lhs = self.parse_lhs_expression_or_higher()?;
        // ++ -- postfix update expressions
        // if self.cur_kind().is_update_operator() && !self.cur_token().is_on_new_line {
        //     ...
        // }
        Ok(lhs)
    }

    /// Section 13.3 Left-Hand-Side Expression
    pub(crate) fn parse_lhs_expression_or_higher(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let lhs = self.parse_member_expression_or_higher()?;
        let lhs = self.parse_call_expression_rest(span, lhs)?;
        Ok(lhs)
    }

    /// Section 13.3 Call Expression
    fn parse_call_expression_rest(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut lhs = lhs;
        loop {
            lhs = self.parse_member_expression_rest(lhs_span, lhs)?;

            if self.at(Type::LParen) {
                lhs = self.parse_call_arguments(lhs_span, lhs)?;
                continue;
            }
            break;
        }
        Ok(lhs)
    }

    fn parse_call_arguments(
        &mut self,
        lhs_span: Span,
        callee: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.expect(Type::LParen)?;
        let mut arguments = Vec::new();
        while !self.at(Type::RParen) {
            let argument = self.parse_assignment_expression_or_higher()?;
            arguments.push(argument);
            if self.at(Type::Comma) {
                self.bump_any();
            }
        }
        self.expect(Type::RParen)?;
        Ok(Expression::CallExpression(Box::new(CallExpression {
            span: self.end_span(lhs_span),
            callee,
            arguments,
        })))
    }

    /// Section 13.3 Member Expression
    fn parse_member_expression_or_higher(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let lhs = self.parse_primary_expression()?;
        self.parse_member_expression_rest(span, lhs)
    }

    /// parse rhs of a member expression, starting from lhs
    #[allow(unused_variables)]
    fn parse_member_expression_rest(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut lhs = lhs;
        loop {
            lhs = match self.cur_kind() {
                Type::Dot => self.parse_static_member_expression(lhs_span, lhs)?,
                Type::LBrack => self.parse_computed_member_expression(lhs_span, lhs)?,
                _ => break,
            };
        }
        Ok(lhs)
    }

    fn parse_static_member_expression(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.bump_any(); // advance `.`
        let ident = self.parse_identifier_name()?;
        Ok(Expression::StaticMemberExpression(Box::new(
            StaticMemberExpression {
                span: self.end_span(lhs_span),
                object: lhs,
                property: ident,
            },
        )))
    }

    fn parse_computed_member_expression(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.bump_any(); // advance `[`
        let property = self.parse_expr()?;
        self.expect(Type::RBrack)?;
        Ok(Expression::ComputedMemberExpression(Box::new(
            ComputedMemberExpression {
                span: self.end_span(lhs_span),
                object: lhs,
                expression: property,
            },
        )))
    }

    #[allow(unused_variables)]
    fn parse_primary_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();

        match self.cur_kind() {
            Type::Identifier => self.parse_identifier_expression(),
            kind if kind.is_literal() => self.parse_literal_expression(),
            Type::LBrack => self.parse_array_expression(),
            Type::LCurly => self.parse_object_expression(),
            Type::LParen => self.parse_parenthesized_expression(span),
            _ => self.parse_identifier_expression(),
        }
    }

    fn parse_parenthesized_expression(&mut self, span: Span) -> Result<Expression<'a>> {
        self.expect(Type::LParen)?;
        let mut expressions = vec![];
        while !self.at(Type::RParen) {
            let expression = self.parse_assignment_expression_or_higher()?;
            expressions.push(expression);
            if self.at(Type::Comma) {
                self.bump_any();
            }
        }
        self.expect(Type::RParen)?;

        let paren_span = self.end_span(span);

        if expressions.is_empty() {
            return Err("Parenthesized expression must contain at least one expression".into());
        }

        // ParenthesizedExpression is from acorn --preserveParens
        if expressions.len() == 1 {
            let expression = expressions.remove(0);
            Ok(Expression::ParenthesizedExpression(Box::new(
                ParenthesizedExpression {
                    span: paren_span,
                    expression,
                },
            )))
        } else {
            Ok(Expression::SequenceExpression(Box::new(
                SequenceExpression {
                    span: paren_span,
                    expressions,
                },
            )))
        }
    }

    pub(crate) fn parse_array_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();

        self.expect(Type::LBrack)?;
        let mut elements = Vec::new();

        while !self.at(Type::RBrack) {
            let span = self.start_span();
            if self.eat(Type::Comma) {
                elements.push(ArrayExpressionElement::Elision(Elision { span }));
                continue;
            }

            let element = self.parse_assignment_expression_or_higher()?;
            elements.push(ArrayExpressionElement::Expression(element));

            if self.at(Type::Comma) {
                self.bump_any();
            }
        }

        self.expect(Type::RBrack)?;

        Ok(Expression::ArrayExpression(Box::new(ArrayExpression {
            span: self.end_span(span),
            elements,
        })))
    }

    fn parse_array_expression_element(&mut self) -> Result<ArrayExpressionElement<'a>> {
        match self.cur_kind() {
            Type::Comma => Ok(ArrayExpressionElement::Elision(Elision {
                span: self.start_span(),
            })),
            _ => self
                .parse_assignment_expression_or_higher()
                .map(ArrayExpressionElement::Expression),
        }
    }

    pub(crate) fn parse_literal_expression(&mut self) -> Result<Expression<'a>> {
        match self.cur_kind() {
            Type::Str => self
                .parse_literal_string()
                .map(|literal| Expression::StringLiteral(Box::new(literal))),
            Type::True | Type::False => self
                .parse_literal_boolean()
                .map(|literal| Expression::BooleanLiteral(Box::new(literal))),
            Type::Null => {
                let literal = self.parse_literal_null();
                Ok(Expression::NullLiteral(Box::new(literal)))
            }
            kind if kind.is_number() => self
                .parse_literal_number()
                .map(|literal| Expression::NumericLiteral(Box::new(literal))),
            _ => Err(self.unexpected()),
        }
    }

    pub(crate) fn parse_literal_string(&mut self) -> Result<StringLiteral<'a>> {
        if !self.at(Type::Str) {
            return Err(self.unexpected());
        }
        let value = self.cur_string();
        let span = self.start_span();
        self.bump_any();
        Ok(StringLiteral { span, value })
    }

    pub(crate) fn parse_literal_boolean(&mut self) -> Result<BooleanLiteral> {
        let span = self.start_span();
        let value = match self.cur_kind() {
            Type::True => true,
            Type::False => false,
            _ => return Err(self.unexpected()),
        };
        self.bump_any();

        Ok(BooleanLiteral { span, value })
    }

    pub(crate) fn parse_literal_null(&mut self) -> NullLiteral {
        let span = self.start_span();
        self.bump_any(); // bump `null`
        NullLiteral { span }
    }

    pub(crate) fn parse_literal_number(&mut self) -> Result<NumericLiteral<'a>> {
        if !self.cur_kind().is_number() {
            return Err(self.unexpected());
        }

        // TODO: implement our own number parser
        use std::num::ParseFloatError;

        let raw = self.cur_string();
        let value = raw
            .parse::<f64>()
            .map_err(|err: ParseFloatError| format!("Failed to parse number: {:?}", err))?;
        let span = self.start_span();
        self.bump_any();
        Ok(NumericLiteral { span, value, raw })
    }

    fn parse_unary_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let operator = map_unary_operator(self.cur_kind());
        self.bump_any();
        let argument = self.parse_simple_unary_expression(span)?;
        Ok(Expression::UnaryExpression(Box::new(UnaryExpression {
            span: self.end_span(span),
            operator,
            argument,
        })))
    }

    pub(crate) fn parse_binary_expression_or_higher(
        &mut self,
        lhs_precedence: Precedence,
    ) -> Result<Expression<'a>> {
        let lhs_span = self.start_span();

        let lhs = self.parse_unary_expression_or_higher(lhs_span)?;

        self.parse_binary_expression_rest(lhs_span, lhs, lhs_precedence)
    }

    fn parse_binary_expression_rest(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
        min_precedence: Precedence,
    ) -> Result<Expression<'a>> {
        // Pratt Parsing
        // https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
        let mut lhs = lhs;
        loop {
            let kind = self.cur_kind();

            let Some(left_precedence) = kind_to_precedence(kind) else {
                break;
            };

            let stop = if left_precedence.is_right_associative() {
                left_precedence < min_precedence
            } else {
                left_precedence <= min_precedence
            };

            if stop {
                break;
            }

            // TODO:
            // Omit the In keyword for the grammar in 13.10 Relational Operators
            // RelationalExpression[In, Yield, Await] :
            // [+In] RelationalExpression[+In, ?Yield, ?Await] in ShiftExpression[?Yield, ?Await]
            // if kind == Type::In && !self.ctx.has_in() {
            //     break;
            // }

            self.bump_any(); // bump operator
            let rhs = self.parse_binary_expression_or_higher(left_precedence)?;

            lhs = if kind.is_logical_operator() {
                Expression::LogicalExpression(Box::new(LogicalExpression {
                    span: self.end_span(lhs_span),
                    left: lhs,
                    operator: map_logical_operator(kind),
                    right: rhs,
                }))
            } else if kind.is_binary_operator() {
                Expression::BinaryExpression(Box::new(BinaryExpression {
                    span: self.end_span(lhs_span),
                    left: lhs,
                    operator: map_binary_operator(kind),
                    right: rhs,
                }))
            } else {
                break;
            };
        }

        Ok(lhs)
    }

    pub(crate) fn parse_identifier_expression(&mut self) -> Result<Expression<'a>> {
        let ident = self.parse_identifier()?;
        Ok(Expression::Identifier(Box::new(ident)))
    }

    pub(crate) fn parse_identifier_name(&mut self) -> Result<IdentifierName<'a>> {
        let (span, name) = self.parse_identifier_kind();
        Ok(IdentifierName { span, name })
    }

    pub(crate) fn parse_identifier(&mut self) -> Result<Identifier<'a>> {
        if !matches!(self.cur_kind(), Type::Identifier) {
            return Err(self.unexpected());
        }
        let (span, name) = self.parse_identifier_kind();
        // self.check_identifier(span, &name);
        Ok(Identifier { span, name })
    }

    #[inline]
    pub(crate) fn parse_identifier_kind(&mut self) -> (Span, &'a str) {
        let span = self.start_span();
        let name = self.cur_string();
        // self.bump_remap(kind);
        self.advance();
        (self.end_span(span), name)
    }
}
