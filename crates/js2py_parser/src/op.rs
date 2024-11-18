use crate::syntax::{
    operator::{
        AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
    },
    precedence::Precedence,
};

use super::Type;

pub fn kind_to_precedence(kind: Type) -> Option<Precedence> {
    match kind {
        // Type::Question2 => Some(Precedence::NullishCoalescing),
        Type::Pipe2 => Some(Precedence::LogicalOr),
        Type::Amp2 => Some(Precedence::LogicalAnd),
        Type::Pipe => Some(Precedence::BitwiseOr),
        Type::Caret => Some(Precedence::BitwiseXor),
        Type::Amp => Some(Precedence::BitwiseAnd),
        Type::Eq2 | Type::Eq3 | Type::Neq | Type::Neq2 => Some(Precedence::Equals),
        Type::LAngle | Type::RAngle | Type::LtEq | Type::GtEq | Type::Instanceof | Type::In => {
            Some(Precedence::Compare)
        }
        Type::ShiftLeft | Type::ShiftRight | Type::ShiftRight3 => Some(Precedence::Shift),
        Type::Plus | Type::Minus => Some(Precedence::Add),
        Type::Star | Type::Slash | Type::Percent => Some(Precedence::Multiply),
        // Type::Star2 => Some(Precedence::Exponentiation),
        // Type::As | Type::Satisfies if is_typescript => Some(Precedence::Compare),
        _ => None,
    }
}

pub fn map_binary_operator(kind: Type) -> BinaryOperator {
    match kind {
        Type::Eq2 => BinaryOperator::Equality,
        Type::Neq => BinaryOperator::Inequality,
        Type::Eq3 => BinaryOperator::StrictEquality,
        Type::Neq2 => BinaryOperator::StrictInequality,
        Type::LAngle => BinaryOperator::LessThan,
        Type::LtEq => BinaryOperator::LessEqualThan,
        Type::RAngle => BinaryOperator::GreaterThan,
        Type::GtEq => BinaryOperator::GreaterEqualThan,
        Type::ShiftLeft => BinaryOperator::ShiftLeft,
        Type::ShiftRight => BinaryOperator::ShiftRight,
        Type::ShiftRight3 => BinaryOperator::ShiftRightZeroFill,
        Type::Plus => BinaryOperator::Addition,
        Type::Minus => BinaryOperator::Subtraction,
        Type::Star => BinaryOperator::Multiplication,
        Type::Slash => BinaryOperator::Division,
        Type::Percent => BinaryOperator::Remainder,
        Type::Pipe => BinaryOperator::BitwiseOR,
        Type::Caret => BinaryOperator::BitwiseXOR,
        Type::Amp => BinaryOperator::BitwiseAnd,
        Type::In => BinaryOperator::In,
        Type::Instanceof => BinaryOperator::Instanceof,
        // Type::Star2 => BinaryOperator::Exponential,
        _ => unreachable!("Binary Operator: {kind:?}"),
    }
}

pub fn map_unary_operator(kind: Type) -> UnaryOperator {
    match kind {
        Type::Minus => UnaryOperator::UnaryNegation,
        Type::Plus => UnaryOperator::UnaryPlus,
        Type::Bang => UnaryOperator::LogicalNot,
        Type::Tilde => UnaryOperator::BitwiseNot,
        Type::Typeof => UnaryOperator::Typeof,
        Type::Void => UnaryOperator::Void,
        Type::Delete => UnaryOperator::Delete,
        _ => unreachable!("Unary Operator: {kind:?}"),
    }
}

pub fn map_logical_operator(kind: Type) -> LogicalOperator {
    match kind {
        Type::Pipe2 => LogicalOperator::Or,
        Type::Amp2 => LogicalOperator::And,
        // Type::Question2 => LogicalOperator::Coalesce,
        _ => unreachable!("Logical Operator: {kind:?}"),
    }
}

pub fn map_update_operator(kind: Type) -> UpdateOperator {
    match kind {
        Type::Plus2 => UpdateOperator::Increment,
        Type::Minus2 => UpdateOperator::Decrement,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}

pub fn map_assignment_operator(kind: Type) -> AssignmentOperator {
    match kind {
        Type::Eq => AssignmentOperator::Assign,
        Type::PlusEq => AssignmentOperator::Addition,
        Type::MinusEq => AssignmentOperator::Subtraction,
        Type::StarEq => AssignmentOperator::Multiplication,
        Type::SlashEq => AssignmentOperator::Division,
        Type::PercentEq => AssignmentOperator::Remainder,
        Type::ShiftLeftEq => AssignmentOperator::ShiftLeft,
        Type::ShiftRightEq => AssignmentOperator::ShiftRight,
        Type::ShiftRight3Eq => AssignmentOperator::ShiftRightZeroFill,
        Type::PipeEq => AssignmentOperator::BitwiseOR,
        Type::CaretEq => AssignmentOperator::BitwiseXOR,
        Type::AmpEq => AssignmentOperator::BitwiseAnd,
        // Type::Amp2Eq => AssignmentOperator::LogicalAnd,
        // Type::Pipe2Eq => AssignmentOperator::LogicalOr,
        // Type::Question2Eq => AssignmentOperator::LogicalNullish,
        // Type::Star2Eq => AssignmentOperator::Exponential,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}
