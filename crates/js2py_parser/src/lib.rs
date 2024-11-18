pub mod ast;
pub mod expr;
pub mod func;
pub mod obj;
pub mod op;
pub mod parser;
pub mod stmt;
pub mod syntax;

pub use js2py_lexer::token::*;
pub use parser::*;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StatementContext {
    If,
    Do,
    While,
    With,
    For,
    StatementList,
}
