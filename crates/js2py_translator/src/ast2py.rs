use super::*;
use js2py_parser::{ast::*, syntax::operator::*};
use std::sync::Arc;

pub struct Ast2PyReturn {
    pub code: String,
}

pub struct Ast2Py {
    source: Arc<str>,
    indent: usize,
}

impl Default for Ast2Py {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast2Py {
    pub fn new() -> Self {
        Self {
            source: "".into(),
            indent: 4,
        }
    }
    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }
    pub fn build(mut self, p: &Program) -> Ast2PyReturn {
        self.source = p.source_text.into();
        let code = self.translate_program(p);
        Ast2PyReturn { code }
    }
}

impl Ast2Py {
    // helper functions:
    fn source_of(&self, node: &impl GetSpan) -> &str {
        &self.source[node.span().start..node.span().end]
    }

    // translate functions:
    fn translate_program(&self, program: &Program) -> String {
        self.translate_statements(&program.body)
    }

    #[rustfmt::skip]
    fn translate_statement(&self, statement: &Statement) -> String {
        match statement {
            Statement::BlockStatement(b) => self.translate_block_statement(b),
            Statement::IfStatement(i) => self.translate_if_statement(i),
            Statement::EmptyStatement(_) => "".into(),
            Statement::FunctionDeclaration(f) => self.translate_function(f),
            Statement::ReturnStatement(r) => self.translate_return_statement(r),
            Statement::VariableDeclarationStatement(v) => self.translate_variable_declaration(v),
            Statement::WhileStatement(w) => self.translate_while_statement(w),
            Statement::ExpressionStatement(e) => self.translate_expression(&e.expression),
            Statement::ContinueStatement(_) => String::from("continue"),
            Statement::BreakStatement(_) => String::from("break"),
            _ => unimplemented!("unsupported statement {:?}", self.source_of(statement)),
        }
    }

    fn translate_statements(&self, stmts: &[Statement<'_>]) -> String {
        stmts
            .iter()
            .filter(|stmt| !matches!(stmt, Statement::EmptyStatement(_)))
            .map(|stmt| self.translate_statement(stmt))
            .collect::<Vec<_>>()
            .join("\n")
            .trim_end()
            .to_string()
    }

    fn translate_if_statement(&self, if_statement: &IfStatement) -> String {
        let test = self.translate_expression(&if_statement.test);
        let consequent = make_indent(
            &self
                .translate_statement(&if_statement.consequent)
                .with_placeholder("pass"),
            self.indent,
        );
        let alternate = if let Some(alt) = &if_statement.alternate {
            format!(
                "\nelse:\n{}",
                make_indent(
                    &self.translate_statement(alt).with_placeholder("pass"),
                    self.indent
                )
            )
        } else {
            String::new()
        };
        format!("if {}:\n{}{}", test, consequent, alternate)
    }

    fn translate_block_statement(&self, block_stmt: &BlockStatement) -> String {
        self.translate_statements(&block_stmt.body)
    }

    fn translate_while_statement(&self, while_stmt: &WhileStatement) -> String {
        let test = self.translate_expression(&while_stmt.test);
        let body = self
            .translate_statement(&while_stmt.body)
            .with_placeholder("pass");
        format!("while {}:\n{}", test, make_indent(&body, self.indent))
    }

    fn translate_variable_declaration(&self, var: &VariableDeclaration) -> String {
        let name = var.id.name.to_string();
        let value = self.translate_expression(var.init.as_ref().unwrap());
        format!("{} = {}", name, value)
    }

    fn translate_function(&self, function: &Function) -> String {
        let name = function
            .id
            .as_ref()
            .map(|id| id.name.to_string())
            .unwrap_or_else(|| unimplemented!("anonymous function is not supported"));

        let params = function
            .params
            .params
            .iter()
            .map(|param| param.name.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let body = function
            .body
            .as_ref()
            .map(|body| self.translate_statements(&body.statements))
            .unwrap_or_default()
            .with_placeholder("pass");

        format!(
            "def {}({}):\n{}",
            name,
            params,
            make_indent(&body, self.indent)
        )
    }

    fn translate_return_statement(&self, ret_stmt: &ReturnStatement) -> String {
        if let Some(argument) = &ret_stmt.argument {
            format!("return {}", self.translate_expression(argument))
        } else {
            String::from("return")
        }
    }

    #[rustfmt::skip]
    fn translate_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::BooleanLiteral(b) => (if b.value { "True" } else { "False" }).to_string(),
            Expression::NumericLiteral(num) => num.value.to_string(),
            Expression::StringLiteral(s) => s.value.to_string(),
            Expression::Identifier(id) => id.name.to_string(),
            Expression::UnaryExpression(u) => self.translate_unary_expression(u),
            Expression::BinaryExpression(bin_expr) => self.translate_binary_expression(bin_expr),
            Expression::StaticMemberExpression(mem_expr) => self.translate_static_member_expression(mem_expr),
            Expression::ComputedMemberExpression(mem_expr) => self.translate_computed_member_expression(mem_expr),
            Expression::ArrayExpression(arr_expr) => self.translate_array_expression(arr_expr),
            Expression::AssignmentExpression(assign_expr) => self.translate_assignment_expression(assign_expr),
            Expression::ObjectExpression(obj_expr) => self.translate_object_expression(obj_expr),
            Expression::CallExpression(call_expr) => self.translate_call_expression(call_expr),
            Expression::LogicalExpression(logic_expr) => self.translate_logical_expression(logic_expr),
            Expression::NullLiteral(_) => String::from("None"),
            Expression::ParenthesizedExpression(e) => self.translate_parenthesized_expression(e),
            _ => unimplemented!("unsupported expression {:?}", self.source_of(expr)),
        }
    }

    fn translate_parenthesized_expression(&self, parent_expr: &ParenthesizedExpression) -> String {
        format!("({})", self.translate_expression(&parent_expr.expression))
    }

    fn translate_unary_expression(&self, unary_expr: &UnaryExpression) -> String {
        let operator = self.translate_unary_operator(&unary_expr.operator);
        let argument = self.translate_expression(&unary_expr.argument);
        if operator == "not " {
            return format!("({}{})", operator, argument);
        }
        format!("{}{}", operator, argument)
    }

    fn translate_unary_operator(&self, operator: &UnaryOperator) -> String {
        match operator {
            UnaryOperator::LogicalNot => "not ".to_string(),
            UnaryOperator::UnaryPlus => "+".to_string(),
            UnaryOperator::UnaryNegation => "0".to_string(),
            UnaryOperator::BitwiseNot => "~".to_string(),
            _ => unimplemented!(
                "unsupported unary operator {:?}",
                serde_json::to_string(operator).unwrap()
            ),
        }
    }

    fn translate_computed_member_expression(&self, mem_expr: &ComputedMemberExpression) -> String {
        let object = self.translate_expression(&mem_expr.object);
        let expression = self.translate_expression(&mem_expr.expression);
        format!("{}[{}]", object, expression)
    }

    fn translate_logical_expression(&self, logic_expr: &LogicalExpression) -> String {
        let left = self.translate_expression(&logic_expr.left);
        let right = self.translate_expression(&logic_expr.right);
        let operator = match logic_expr.operator {
            LogicalOperator::Or => "or",
            LogicalOperator::And => "and",
            _ => unimplemented!(
                "unsupported logical operator {:?}",
                self.source_of(logic_expr)
            ), // Python 没有 ?? 操作符
        };
        format!("{} {} {}", left, operator, right)
    }

    fn translate_call_expression(&self, call_expr: &CallExpression) -> String {
        let callee = self.translate_expression(&call_expr.callee);

        let arguments = call_expr
            .arguments
            .iter()
            .map(|arg| self.translate_expression(arg))
            .collect::<Vec<_>>()
            .join(", ");

        // 特殊判断：如果 callee 是 StaticMemberExpression，且 property 为 push
        if let Expression::StaticMemberExpression(mem_expr) = &call_expr.callee {
            if let Expression::Identifier(id) = &mem_expr.object {
                if id.name == "console" && mem_expr.property.name == "log" {
                    return format!("print({})", arguments);
                }
            }
            if mem_expr.property.name == "push" {
                let object = self.translate_expression(&mem_expr.object);
                return format!("{}.append({})", object, arguments);
            }
        }

        if let Expression::Identifier(id) = &call_expr.callee {
            if id.name == "parseFloat" {
                return format!("float({})", arguments);
            }
        }

        format!("{}({})", callee, arguments)
    }

    fn translate_object_expression(&self, obj_expr: &ObjectExpression) -> String {
        let properties = obj_expr
            .properties
            .iter()
            .map(|prop| {
                let var_name = match &prop.key {
                    PropertyKey::IdentifierName(id) => format!("\"{}\"", id.name),
                    PropertyKey::StringLiteral(s) => s.value.to_string(),
                    PropertyKey::NumericLiteral(n) => n.value.to_string(),
                };
                let key = var_name;
                let value = self.translate_expression(&prop.value);
                format!("{}: {}", key, value)
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{{}}}", properties)
    }

    fn translate_array_expression(&self, arr_expr: &ArrayExpression) -> String {
        let elements = arr_expr
            .elements
            .iter()
            .map(|element| match element {
                ArrayExpressionElement::Elision(_) => "None".to_string(),
                ArrayExpressionElement::Expression(expr) => self.translate_expression(expr),
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{}]", elements)
    }

    fn translate_static_member_expression(&self, mem_expr: &StaticMemberExpression) -> String {
        let object = self.translate_expression(&mem_expr.object);
        let property = mem_expr.property.name.to_string();
        if property == "length" {
            return format!("len({})", object);
        }
        format!("{}.{}", object, property)
    }

    fn translate_binary_expression(&self, bin_expr: &BinaryExpression) -> String {
        let lhs = self.translate_expression(&bin_expr.left);
        let rhs = self.translate_expression(&bin_expr.right);
        let op = self.translate_binary_operator(bin_expr.operator);
        format!("{} {} {}", lhs, op, rhs)
    }

    fn translate_assignment_expression(&self, assign_expr: &AssignmentExpression) -> String {
        let left = match &assign_expr.left {
            AssignmentTarget::Identifier(id) => id.name.to_string(),
            AssignmentTarget::StaticMemberExpression(mem_expr) => {
                self.translate_static_member_expression(mem_expr)
            }
            AssignmentTarget::ComputedMemberExpression(mem_expr) => {
                self.translate_computed_member_expression(mem_expr)
            }
        };
        let operator = self.translate_assign_operator(&assign_expr.operator);
        let right = self.translate_expression(&assign_expr.right);
        format!("{} {} {}", left, operator, right)
    }

    fn translate_assign_operator(&self, operator: &AssignmentOperator) -> String {
        use AssignmentOperator::*;
        match operator {
            ShiftRightZeroFill | LogicalAnd | LogicalOr | LogicalNullish => unimplemented!(
                "unsupported assignment operator {:?}",
                serde_json::to_string(&operator).unwrap()
            ),
            o => serde_json::to_string(o)
                .unwrap()
                .trim_matches('"')
                .to_string(),
        }
    }

    fn translate_binary_operator(&self, operator: BinaryOperator) -> String {
        use BinaryOperator::*;
        match operator {
            StrictEquality => "is",
            StrictInequality => "is not",
            Equality => "==",
            Inequality => "!=",
            LessThan => "<",
            GreaterThan => ">",
            Addition => "+",
            Subtraction => "-",
            Multiplication => "*",
            Division => "/",
            Remainder => "%",
            BitwiseOR => "|",
            BitwiseXOR => "^",
            LessEqualThan => "<=",
            GreaterEqualThan => ">=",
            ShiftLeft => "<<",
            ShiftRight => ">>",
            BitwiseAnd => "&",
            _ => unimplemented!(
                "unsupported binary operator {:?}",
                serde_json::to_string(&operator).unwrap()
            ),
        }
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use js2py_parser::Parser;
    fn assert_translate(source: &str, expected: &str) {
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();
        let python_code = super::Ast2Py::default().build(&ast).code;
        assert_eq!(python_code, expected);
    }
    #[test]
    fn test_if_statement_without_curly_braces() {
        let source = "if (a) b";
        let expected = "if a:\n    b";
        assert_translate(source, expected);
    }
    #[test]
    fn test_else_if_statement_without_curly_braces() {
        let source = "if (a) b; else if (c) d;";
        let expected = "if a:\n    b\nelse:\n    if c:\n        d";
        assert_translate(source, expected);
    }
    #[test]
    fn test_while_statement_without_curly_braces() {
        let source = "while (a) b";
        let expected = "while a:\n    b";
        assert_translate(source, expected);
    }
    #[test]
    fn test_parenthesized_expression() {
        assert_translate("(1 + 2) * 3", "(1 + 2) * 3");
    }
    #[test]
    fn test_simple_function() {
        let source = "function foo() {return}";
        let expected = "def foo():\n    return";
        assert_translate(source, expected);
    }
    #[test]
    fn test_empty_function() {
        let source = "function foo() {}";
        let expected = "def foo():\n    pass";
        assert_translate(source, expected);
    }
    #[test]
    fn test_empty_while() {
        let source = "while (true);";
        let expected = "while True:\n    pass";
        assert_translate(source, expected);
    }
    #[test]
    fn test_empty_if_else() {
        let source = "if (true){}else{};";
        let expected = "if True:\n    pass\nelse:\n    pass";
        assert_translate(source, expected);
    }
}
