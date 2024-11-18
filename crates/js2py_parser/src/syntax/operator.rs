use serde::Serialize;

// #![allow(non_snake_case)]

/// Operators that may be used in assignment epxressions.
///
/// ## References
/// - [13.15 Assignment Operators](https://tc39.es/ecma262/#sec-assignment-operators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AssignmentOperator {
    #[serde(rename = "=")]
    Assign = 0,
    #[serde(rename = "+=")]
    Addition = 1,
    #[serde(rename = "-=")]
    Subtraction = 2,
    #[serde(rename = "*=")]
    Multiplication = 3,
    #[serde(rename = "/=")]
    Division = 4,
    #[serde(rename = "%=")]
    Remainder = 5,
    #[serde(rename = "<<=")]
    ShiftLeft = 6,
    #[serde(rename = ">>=")]
    ShiftRight = 7,
    #[serde(rename = ">>>=")]
    ShiftRightZeroFill = 8,
    #[serde(rename = "|=")]
    BitwiseOR = 9,
    #[serde(rename = "^=")]
    BitwiseXOR = 10,
    #[serde(rename = "&=")]
    BitwiseAnd = 11,
    #[serde(rename = "&&=")]
    LogicalAnd = 12,
    #[serde(rename = "||=")]
    LogicalOr = 13,
    #[serde(rename = "??=")]
    LogicalNullish = 14,
    #[serde(rename = "**=")]
    Exponential = 15,
}

/// Operators used in binary expressions. Does not include logical binary
/// operators, which are in [`LogicalOperator`].
///
/// ## References
/// - [12.10 Binary Logical Operators](https://tc39.es/ecma262/#sec-binary-logical-operators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum BinaryOperator {
    #[serde(rename = "==")]
    Equality = 0,
    #[serde(rename = "!=")]
    Inequality = 1,
    #[serde(rename = "===")]
    StrictEquality = 2,
    #[serde(rename = "!==")]
    StrictInequality = 3,
    #[serde(rename = "<")]
    LessThan = 4,
    #[serde(rename = "<=")]
    LessEqualThan = 5,
    #[serde(rename = ">")]
    GreaterThan = 6,
    #[serde(rename = ">=")]
    GreaterEqualThan = 7,
    #[serde(rename = "<<")]
    ShiftLeft = 8,
    #[serde(rename = ">>")]
    ShiftRight = 9,
    #[serde(rename = ">>>")]
    ShiftRightZeroFill = 10,
    #[serde(rename = "+")]
    Addition = 11,
    #[serde(rename = "-")]
    Subtraction = 12,
    #[serde(rename = "*")]
    Multiplication = 13,
    #[serde(rename = "/")]
    Division = 14,
    #[serde(rename = "%")]
    Remainder = 15,
    #[serde(rename = "|")]
    BitwiseOR = 16,
    #[serde(rename = "^")]
    BitwiseXOR = 17,
    #[serde(rename = "&")]
    BitwiseAnd = 18,
    #[serde(rename = "in")]
    In = 19,
    #[serde(rename = "instanceof")]
    Instanceof = 20,
    #[serde(rename = "**")]
    Exponential = 21,
}

/// Logical binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LogicalOperator {
    #[serde(rename = "||")]
    Or = 0,
    #[serde(rename = "&&")]
    And = 1,
    #[serde(rename = "??")]
    Coalesce = 2,
}

/// Operators used in unary operators.
///
/// Does not include self-modifying operators, which are in [`UpdateOperator`].
///
/// ## References
/// - [12.5 Unary Operators](https://tc39.es/ecma262/#sec-unary-operators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum UnaryOperator {
    #[serde(rename = "-")]
    UnaryNegation = 0,
    #[serde(rename = "+")]
    UnaryPlus = 1,
    #[serde(rename = "!")]
    LogicalNot = 2,
    #[serde(rename = "~")]
    BitwiseNot = 3,
    #[serde(rename = "typeof")]
    Typeof = 4,
    #[serde(rename = "void")]
    Void = 5,
    #[serde(rename = "delete")]
    Delete = 6,
}

/// Unary update operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum UpdateOperator {
    #[serde(rename = "++")]
    Increment = 0,
    #[serde(rename = "--")]
    Decrement = 1,
}
