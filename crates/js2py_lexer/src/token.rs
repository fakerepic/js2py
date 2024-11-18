#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Token {
    pub typ: Type,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Type {
    // keywords
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,

    // Contextual Keywords
    Async,
    From,
    Get,
    Meta, // import.meta
    Of,
    Set,
    Target,   // new.target
    Accessor, // proposal-decorators

    // Future keywords (strict mode reserved words)
    Implements,
    Interface,
    Let,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Yield,

    // punctuator
    Amp,           // &
    Amp2,          // &&
    AmpEq,         // &=
    Bang,          // !
    Caret,         // ^
    CaretEq,       // ^=
    Colon,         // :
    Comma,         // ,
    Dot,           // .
    Eq,            // =
    Eq2,           // ==
    Eq3,           // ===
    GtEq,          // >=
    LAngle,        // <
    LBrack,        // [
    LCurly,        // {
    LParen,        // (
    LtEq,          // <=
    Minus,         // -
    Minus2,        // --
    MinusEq,       // -=
    Neq,           // !=
    Neq2,          // !==
    Percent,       // %
    PercentEq,     // %=
    Pipe,          // |
    Pipe2,         // ||
    PipeEq,        // |=
    Plus,          // +
    Plus2,         // ++
    PlusEq,        // +=
    Question,      // ?
    RAngle,        // >
    RBrack,        // ]
    RCurly,        // }
    RParen,        // )
    Semicolon,     // ;
    ShiftLeft,     // <<
    ShiftLeftEq,   // <<=
    ShiftRight,    // >>
    ShiftRightEq,  // >>=
    ShiftRight3,   // >>>
    ShiftRight3Eq, // >>>=
    Star,          // *
    StarEq,        // *=
    Tilde,         // ~
    // div punctuator
    Slash,   // /
    SlashEq, // /=

    // Literals
    // Null Literals
    Null,
    // Boolean Literals
    True,
    False,
    // Numeric Literals
    Decimal,
    Hex,
    // String Literals
    /// String Type
    Str,

    // Other
    Identifier,
    LineTerminator,
    EOF,
    LexerError,
}
use self::Type::*;

impl Type {
    pub fn match_keyword(s: &str) -> Self {
        let len = s.len();
        if len <= 1 || len >= 12 || !s.as_bytes()[0].is_ascii_lowercase() {
            return Type::Identifier;
        }
        Self::match_keyword_impl(s)
    }

    pub fn to_str(self) -> &'static str {
        match self {
            EOF => "eof",
            Identifier => "identifier",
            Await => "await",
            Break => "break",
            Case => "case",
            Catch => "catch",
            Class => "class",
            Const => "const",
            Continue => "continue",
            Debugger => "debugger",
            Default => "default",
            Delete => "delete",
            Do => "do",
            Else => "else",
            Enum => "enum",
            Export => "export",
            Extends => "extends",
            Finally => "finally",
            For => "for",
            Function => "function",
            If => "if",
            Import => "import",
            In => "in",
            Instanceof => "instanceof",
            New => "new",
            Return => "return",
            Super => "super",
            Switch => "switch",
            This => "this",
            Throw => "throw",
            Try => "try",
            Typeof => "typeof",
            Var => "var",
            Void => "void",
            While => "while",
            With => "with",
            Async => "async",
            From => "from",
            Get => "get",
            Meta => "meta",
            Of => "of",
            Set => "set",
            Accessor => "accessor",
            Target => "target",
            Implements => "implements",
            Interface => "interface",
            Package => "package",
            Private => "private",
            Protected => "protected",
            Public => "public",
            Static => "static",
            Let => "let",
            Yield => "yield",
            Amp => "&",
            Amp2 => "&&",
            AmpEq => "&=",
            Bang => "!",
            Caret => "^",
            CaretEq => "^=",
            Colon => ":",
            Comma => ",",
            Dot => ".",
            Eq => "=",
            Eq2 => "==",
            Eq3 => "===",
            GtEq => ">=",
            LAngle => "<",
            LBrack => "[",
            LCurly => "{",
            LParen => "(",
            LtEq => "<=",
            Minus => "-",
            Minus2 => "--",
            MinusEq => "-=",
            Neq => "!=",
            Neq2 => "!==",
            Percent => "%",
            PercentEq => "%=",
            Pipe => "|",
            Pipe2 => "||",
            PipeEq => "|=",
            Plus => "+",
            Plus2 => "++",
            PlusEq => "+=",
            Question => "?",
            RAngle => ">",
            RBrack => "]",
            RCurly => "}",
            RParen => ")",
            Semicolon => ";",
            ShiftLeft => "<<",
            ShiftLeftEq => "<<=",
            ShiftRight => ">>",
            ShiftRight3 => ">>>",
            ShiftRight3Eq => ">>>=",
            ShiftRightEq => ">>=",
            Slash => "/",
            SlashEq => "/=",
            Star => "*",
            StarEq => "*=",
            Tilde => "~",
            Null => "null",
            True => "true",
            False => "false",
            Decimal => "decimal",
            Hex => "hex",
            Str => "string",
            LineTerminator => "lineterminator",
            _ => unreachable!("unreachable token type: {:?}", self),
        }
    }

    fn match_keyword_impl(s: &str) -> Self {
        match s {
            "do" => Do,
            "if" => If,
            "in" => In,
            "of" => Of,

            "for" => For,
            "get" => Get,
            "let" => Let,
            "new" => New,
            "set" => Set,
            "try" => Try,
            "var" => Var,

            "case" => Case,
            "else" => Else,
            "enum" => Enum,
            "from" => From,
            "meta" => Meta,
            "null" => Null,
            "this" => This,
            "true" => True,
            "void" => Void,
            "with" => With,

            "async" => Async,
            "await" => Await,
            "break" => Break,
            "catch" => Catch,
            "class" => Class,
            "const" => Const,
            "false" => False,
            "super" => Super,
            "throw" => Throw,
            "while" => While,
            "yield" => Yield,

            "delete" => Delete,
            "export" => Export,
            "import" => Import,
            "public" => Public,
            "return" => Return,
            "static" => Static,
            "switch" => Switch,
            "target" => Target,
            "typeof" => Typeof,

            "default" => Default,
            "extends" => Extends,
            "finally" => Finally,
            "package" => Package,
            "private" => Private,

            "accessor" => Accessor,
            "continue" => Continue,
            "debugger" => Debugger,
            "function" => Function,

            "interface" => Interface,
            "protected" => Protected,

            "implements" => Implements,
            "instanceof" => Instanceof,

            _ => Identifier,
        }
    }
}

impl std::default::Default for Token {
    fn default() -> Self {
        Self {
            typ: Type::EOF,
            start: 0,
            end: 0,
        }
    }
}
