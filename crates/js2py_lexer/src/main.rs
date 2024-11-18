//! usage: ./{{project_name}} <filename> (only for testing, you can use this crate as a library)

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    if !std::path::Path::new(filename).exists() {
        eprintln!("File not found: {}", filename);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let token_stream = js2py_lexer::lexer::token_stream(&input);
    for token in token_stream.iter()
    // .filter(|t| t.typ != js2py_lexer::token::Type::LineTerminator)
    {
        // println!("('{:?}', '{}')", token.typ, &input[token.start..token.end]);
        println!(
            "{:?} \"{}\"",
            token,
            &input[token.start..token.end].replace("\n", "\\n")
        );
    }
}

#[test]
fn test1() {
    use js2py_lexer::token::Type;
    use js2py_lexer::*;
    let input = "var x = 10;";
    let token_stream = lexer::token_stream(input);
    let tokens: Vec<_> = token_stream.iter().collect();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].typ, Type::Var);
    assert_eq!(tokens[1].typ, Type::Identifier);
    assert_eq!(tokens[2].typ, Type::Eq);
    assert_eq!(tokens[3].typ, Type::Decimal);
    assert_eq!(tokens[4].typ, Type::Semicolon);
    assert_eq!(tokens[5].typ, Type::EOF);
}
