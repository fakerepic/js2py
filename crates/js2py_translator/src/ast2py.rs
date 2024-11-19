use super::*;
use js2py_parser::{ast::*, syntax::operator::*};
use std::{fmt::Write, sync::Arc};

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
        let mut result = String::new();
        program
            .body
            .iter()
            .filter(|statement| !matches!(statement, Statement::EmptyStatement(_)))
            .for_each(|statement| {
                let translated = self.translate_statement(statement);
                writeln!(result, "{}", translated).unwrap();
            });
        result
    }

    fn translate_statement(&self, statement: &Statement) -> String {
        match statement {
            Statement::EmptyStatement(_) => "".into(),
            Statement::FunctionDeclaration(f) => self.translate_function(f),
            Statement::ReturnStatement(r) => self.translate_return_statement(r),
            Statement::VariableDeclarationStatement(v) => self.translate_variable_declaration(v),
            _ => unimplemented!("unsupported statement {:?}", self.source_of(statement)),
        }
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
            .unwrap_or_else(|| "anonymous".to_string());

        let params = function
            .params
            .params
            .iter()
            .map(|param| param.name.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let body = if let Some(body) = &function.body {
            body.statements
                .iter()
                .map(|stmt| self.translate_statement(stmt))
                .map(|code| make_indent(&code, self.indent))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            format!("{:indent$}pass", "", indent = self.indent)
        };

        format!("def {}({}):\n{}", name, params, body)
    }

    fn translate_return_statement(&self, ret_stmt: &ReturnStatement) -> String {
        if let Some(argument) = &ret_stmt.argument {
            format!("return {}", self.translate_expression(argument))
        } else {
            String::from("return")
        }
    }

    fn translate_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::NumericLiteral(num) => num.value.to_string(),
            Expression::Identifier(id) => id.name.to_string(),
            Expression::BinaryExpression(bin_expr) => {
                let lhs = self.translate_expression(&bin_expr.left);
                let rhs = self.translate_expression(&bin_expr.right);
                let op = self.translate_binary_operator(bin_expr.operator);
                format!("{} {} {}", lhs, op, rhs)
            }
            _ => unimplemented!("unsupported expression {:?}", self.source_of(expr)),
        }
    }

    fn translate_binary_operator(&self, operator: BinaryOperator) -> &'static str {
        match operator {
            BinaryOperator::Addition => "+",
            BinaryOperator::Subtraction => "-",
            BinaryOperator::Multiplication => "*",
            BinaryOperator::Division => "/",
            _ => unimplemented!("unsupported binary operator {:?}", operator),
        }
    }
}
