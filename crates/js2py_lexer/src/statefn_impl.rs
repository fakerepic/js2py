//! The implementation of the state functions that the lexer uses to parse the input string.

use crate::lexer::Lexer;
use crate::statefn::StateFn;
use crate::token::Type;
use crate::token::Type::*;

macro_rules! sf {
    ($name:ident) => {
        Some(StateFn::from($name))
    };
}

pub fn lex_start(lexer: &mut Lexer) -> Option<StateFn> {
    if lexer.eof() {
        lexer.emit(EOF);
        return None;
    }

    let c = lexer.peek();

    // trivial cases for punctuators
    if let Some(next_f) = match c.unwrap() {
        ':' => Some(Colon),
        ',' => Some(Comma),
        ';' => Some(Semicolon),
        '(' => Some(LParen),
        ')' => Some(RParen),
        '{' => Some(LCurly),
        '}' => Some(RCurly),
        '[' => Some(LBrack),
        ']' => Some(RBrack),
        '~' => Some(Tilde),
        '.' => Some(Dot),
        '?' => Some(Question),
        _ => None,
    } {
        lexer.step();
        lexer.emit(next_f);
        return sf!(lex_start);
    }

    // LineTerminator ::
    //   <LF>
    //   <CR>
    //   <LS>
    //   <PS>
    // line terminators:
    if matches!(c.unwrap(), '\n' | '\r' | '\u{2028}' | '\u{2029}') {
        lexer.step();
        lexer.emit(LineTerminator);
        return sf!(lex_start);
    }

    match c.unwrap() {
        'a'..='z' | 'A'..='Z' | '_' => sf!(lex_identifier_or_keyword),
        '1'..='9' => sf!(lex_decimal),
        '0' => sf!(lex_zero),
        '"' | '\'' => sf!(lex_string_literal),
        '=' => sf!(lex_eq),
        '!' => sf!(lex_bang),
        '|' => sf!(lex_pipe),
        '<' => sf!(lex_lt),
        '>' => sf!(lex_gt),
        '&' => sf!(lex_amp),
        '*' => sf!(lex_star),
        '/' => sf!(lex_slash),
        '+' => sf!(lex_plus),
        '-' => sf!(lex_minus),
        '^' => sf!(lex_caret),
        '%' => sf!(lex_percent),

        // WhiteSpace ::
        //   <TAB>
        //   <VT>
        //   <FF>
        //   <SP>
        //   <NBSP>
        //   <BOM>
        //   <USP>
        ' ' | '\t' | '\x0B' | '\x0C' | '\u{00A0}' | '\u{FEFF}' | '\u{1680}' | '\u{180E}' => {
            lexer.step();
            lexer.ignore();
            sf!(lex_start)
        }
        _ => lex_error(
            lexer,
            format!("Unexpected character: {}", c.unwrap()).as_str(),
        ),
    }
}

fn lex_identifier_or_keyword(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept_run("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_");
    let s = lexer.current();
    let typ = Type::match_keyword(&s);
    lexer.emit(typ);
    sf!(lex_start)
}

fn lex_zero(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    match lexer.peek() {
        Some('x') | Some('X') => {
            lexer.step(); // Skip the x
            if lexer.accept_run("0123456789abcdefABCDEF") {
                lexer.emit(Hex);
                sf!(lex_start)
            } else {
                lex_error(lexer, "Expected hexadecimal number")
            }
        }
        Some('.') => {
            lexer.back();
            sf!(lex_decimal)
        }
        _ => {
            lexer.emit(Decimal); // Emit number
            sf!(lex_start)
        }
    }
}

fn lex_decimal(lexer: &mut Lexer) -> Option<StateFn> {
    let has_int = lexer.accept_run("0123456789");
    if lexer.accept(".") {
        let has_frac = lexer.accept_run("0123456789");
        if !has_int && !has_frac {
            return lex_error(lexer, "Expected decimal number");
        }
    }
    if lexer.accept("eE") {
        lexer.accept("+-");
        lexer.accept_run("0123456789");
    }
    lexer.emit(Decimal);
    sf!(lex_start)
}

fn lex_string_literal(lexer: &mut Lexer) -> Option<StateFn> {
    let quote = lexer.peek().unwrap();
    lexer.step(); // Skip the opening quote
    let check_n_hex = |lexer: &mut Lexer, times: usize| -> bool {
        for _ in 0..times {
            if !lexer.accept_run("0123456789abcdefABCDEF") {
                return false;
            }
        }
        true
    };
    while let Some(c) = lexer.peek() {
        if c == quote {
            lexer.step(); // Skip the closing quote
            lexer.emit(Str);
            return sf!(lex_start);
        } else if c == '\\' {
            lexer.step(); // Skip the backslash
            if let Some(c) = lexer.peek() {
                match c {
                    '\n' | '\r' | '\u{2028}' | '\u{2029}' => {
                        lexer.step(); // Skip the line terminator
                        if c == '\r' && lexer.peek() == Some('\n') {
                            lexer.step(); // Skip the newline
                        }
                        continue;
                    }
                    'b' | 'f' | 'n' | 'r' | 't' | 'v' | '"' | '\'' | '\\' => {
                        lexer.step(); // Skip the escape character
                    }
                    '0' => {
                        lexer.step(); // Skip the zero
                        if let Some(c) = lexer.peek() {
                            if c.is_ascii_digit() {
                                return lex_error(lexer, "Octal literals are not allowed");
                            }
                        }
                    }
                    '1'..='9' => {
                        return lex_error(lexer, "Octal literals are not allowed");
                    }
                    'x' => {
                        lexer.step(); // Skip the x
                        if check_n_hex(lexer, 2) {
                            continue;
                        } else {
                            return lex_error(lexer, "Expected hexadecimal number");
                        }
                    }
                    'u' => {
                        lexer.step(); // Skip the u
                        if check_n_hex(lexer, 4) {
                            continue;
                        } else {
                            return lex_error(lexer, "Expected hexadecimal number");
                        }
                    }
                    _ => {}
                }
            } else {
                return lex_error(lexer, "Unexpected end of input");
            }
        } else if c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}' {
            return lex_error(lexer, "Unexpected line terminator");
        }

        lexer.step();
    }
    lex_error(lexer, "Unexpected end of input")
}

fn lex_eq(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept_run("=");
    match lexer.current().as_str() {
        "===" => lexer.emit(Eq3),
        "==" => lexer.emit(Eq2),
        _ => lexer.emit(Eq),
    }
    sf!(lex_start)
}

fn lex_pipe(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("|") {
        lexer.emit(Pipe2);
    } else if lexer.accept("=") {
        lexer.emit(PipeEq);
    } else {
        lexer.emit(Pipe);
    }
    sf!(lex_start)
}

fn lex_bang(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(Neq);
    } else if lexer.accept("!") {
        lexer.emit(Neq2);
    } else {
        lexer.emit(Bang);
    }
    sf!(lex_start)
}

fn lex_lt(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(LtEq);
    } else if lexer.accept("<") {
        if lexer.accept("=") {
            lexer.emit(ShiftLeftEq);
        } else {
            lexer.emit(ShiftLeft);
        }
    } else {
        lexer.emit(LAngle);
    }
    sf!(lex_start)
}

fn lex_gt(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(GtEq);
    } else if lexer.accept(">") {
        if lexer.accept(">") {
            if lexer.accept("=") {
                lexer.emit(ShiftRight3Eq);
            } else {
                lexer.emit(ShiftRight3);
            }
        } else if lexer.accept("=") {
            lexer.emit(ShiftRightEq);
        } else {
            lexer.emit(ShiftRight);
        }
    } else {
        lexer.emit(RAngle);
    }
    sf!(lex_start)
}

fn lex_amp(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("&") {
        lexer.emit(Amp2);
    } else if lexer.accept("=") {
        lexer.emit(AmpEq);
    } else {
        lexer.emit(Amp);
    }
    sf!(lex_start)
}

fn lex_star(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(StarEq);
    } else {
        lexer.emit(Star);
    }
    sf!(lex_start)
}

fn lex_slash(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(SlashEq);
        sf!(lex_start)
    } else if lexer.accept("/") {
        sf!(lex_single_line_comment)
    } else if lexer.accept("*") {
        sf!(lex_multi_line_comment)
    } else {
        lexer.emit(Slash);
        sf!(lex_start)
    }
}

fn lex_plus(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(PlusEq);
    } else if lexer.accept("+") {
        lexer.emit(Plus2);
    } else {
        lexer.emit(Plus);
    }
    sf!(lex_start)
}

fn lex_minus(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(MinusEq);
    } else if lexer.accept("-") {
        lexer.emit(Minus2);
    } else {
        lexer.emit(Minus);
    }
    sf!(lex_start)
}

fn lex_caret(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(CaretEq);
    } else {
        lexer.emit(Caret);
    }
    sf!(lex_start)
}

fn lex_percent(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if lexer.accept("=") {
        lexer.emit(PercentEq);
    } else {
        lexer.emit(Percent);
    }
    sf!(lex_start)
}

fn lex_single_line_comment(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept("//");
    while let Some(c) = lexer.peek() {
        if c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}' {
            lexer.ignore();
            return sf!(lex_start);
        }
        lexer.step();
    }
    lex_error(lexer, "Unexpected end of input")
}

/// simplified
fn lex_multi_line_comment(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept("/*");
    while let Some(c) = lexer.peek() {
        if c == '*' {
            lexer.step();
            if lexer.accept("/") {
                lexer.ignore();
                return sf!(lex_start);
            }
        }
        lexer.step();
    }
    lex_error(lexer, "Unexpected end of input")
}

// TODO: error handling
fn lex_error(lexer: &mut Lexer, msg: &str) -> Option<StateFn> {
    lexer.send(LexerError);
    lexer.error(msg);
    None
}
