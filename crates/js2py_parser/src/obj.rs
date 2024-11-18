use super::*;
use crate::ast::*;

impl<'a> Parser<'a> {
    pub(crate) fn parse_object_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Type::LCurly)?;
        let mut properties = vec![];
        while !self.at(Type::RCurly) {
            let prop = self.parse_object_property()?;
            properties.push(prop);
            if self.at(Type::Comma) {
                self.bump_any();
            }
        }
        let trailing_comma = self.at(Type::Comma).then(|| self.start_span());
        self.expect(Type::RCurly)?;
        Ok(Expression::ObjectExpression(Box::new(ObjectExpression {
            span: self.end_span(span),
            properties,
            trailing_comma,
        })))
    }
    pub(crate) fn parse_object_property(&mut self) -> Result<ObjectProperty<'a>> {
        let span = self.start_span();
        let key = self.parse_property_key()?;
        self.expect(Type::Colon)?;
        let value = self.parse_assignment_expression_or_higher()?;
        Ok(ObjectProperty {
            span: self.end_span(span),
            key,
            value,
        })
    }
    pub(crate) fn parse_property_key(&mut self) -> Result<PropertyKey<'a>> {
        let key = match self.cur_kind() {
            Type::Identifier => {
                let (span, name) = self.parse_identifier_kind();
                PropertyKey::IdentifierName(IdentifierName { span, name })
            }
            Type::Str => {
                let lit = self.parse_literal_string()?;
                PropertyKey::StringLiteral(lit)
            }
            Type::Decimal => {
                let lit = self.parse_literal_number()?;
                PropertyKey::NumericLiteral(lit)
            }
            _ => {
                return Err(self.unexpected());
            }
        };
        Ok(key)
    }
}
