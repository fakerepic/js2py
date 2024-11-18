use js2py_lexer::token::*;
use Type::*;

#[allow(clippy::wrong_self_convention)]
pub trait TokenTypeUtil {
    fn is_eof(self) -> bool;
    fn is_number(self) -> bool;
    fn is_logical_operator(self) -> bool;
    fn is_binary_operator(self) -> bool;
    fn is_unary_operator(self) -> bool;
    fn is_literal(self) -> bool;
    fn is_reserved_keyword(self) -> bool;
    fn is_identifier(self) -> bool;
    fn is_identifier_name(self) -> bool;
    fn is_strict_mode_contextual_keyword(self) -> bool;
    fn is_contextual_keyword(self) -> bool;
    fn is_future_reserved_keyword(self) -> bool;
    fn is_all_keyword(self) -> bool;
    fn is_variable_declaration(self) -> bool;
    fn is_assignment_operator(self) -> bool;
}

impl TokenTypeUtil for Type {
    fn is_eof(self) -> bool {
        matches!(self, EOF)
    }

    fn is_number(self) -> bool {
        matches!(self, Decimal | Hex)
    }

    fn is_logical_operator(self) -> bool {
        matches!(self, Pipe2 | Amp2)
    }
    #[rustfmt::skip]
    fn is_binary_operator(self) -> bool {
        matches!(self, Eq2 | Neq | Eq3 | Neq2 | LAngle | LtEq | RAngle | GtEq | ShiftLeft | ShiftRight
            | ShiftRight3 | Plus | Minus | Star | Slash | Percent | Pipe | Caret | Amp | In
            | Instanceof)
    }
    fn is_unary_operator(self) -> bool {
        matches!(self, Minus | Plus | Bang | Tilde | Typeof | Void | Delete)
    }
    fn is_literal(self) -> bool {
        matches!(self, Null | True | False | Str) || self.is_number()
    }

    fn is_identifier(self) -> bool {
        self.is_identifier_name() && !self.is_reserved_keyword()
    }

    fn is_identifier_name(self) -> bool {
        matches!(self, Identifier) || self.is_all_keyword()
    }

    fn is_all_keyword(self) -> bool {
        self.is_reserved_keyword()
            || self.is_strict_mode_contextual_keyword()
            || self.is_contextual_keyword()
            || self.is_future_reserved_keyword()
    }

    #[rustfmt::skip]
    fn is_reserved_keyword(self) -> bool {
        matches!(self, Await | Break | Case | Catch | Class | Const | Continue | Debugger | Default
            | Delete | Do | Else | Enum | Export | Extends | False | Finally | For | Function | If
            | Import | In | Instanceof | New | Null | Return | Super | Switch | This | Throw
            | True | Try | Typeof | Var | Void | While | With | Yield)
    }

    #[rustfmt::skip]
    fn is_strict_mode_contextual_keyword(self) -> bool {
        matches!(self, Let | Static | Implements | Interface | Package | Private | Protected | Public)
    }

    #[rustfmt::skip]
    fn is_contextual_keyword(self) -> bool {
        matches!(self, Async | From | Get | Meta | Of | Set | Target | Accessor)
    }

    #[rustfmt::skip]
    fn is_future_reserved_keyword(self) -> bool {
        matches!(self, Implements | Interface | Package | Private | Protected | Public | Static)
    }

    fn is_variable_declaration(self) -> bool {
        matches!(self, Var | Let | Const)
    }
    #[rustfmt::skip]
    fn is_assignment_operator(self) -> bool {
        matches!(self, Eq | PlusEq | MinusEq | StarEq | SlashEq | PercentEq | ShiftLeftEq | ShiftRightEq | ShiftRight3Eq | AmpEq | CaretEq | PipeEq)
    }
}
