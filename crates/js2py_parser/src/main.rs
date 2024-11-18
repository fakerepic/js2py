// usage: ./{{project_name}} <filename>
use js2py_parser::parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./{{project_name}} <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let source = std::fs::read_to_string(filename).unwrap();

    let mut parser = Parser::new(&source);

    let program = parser.parse();

    match program {
        Ok(program) => {
            // Serialize the program to a JSON string and write it to file:
            let path = format!("{}-ast.json", filename);
            let writer = std::fs::File::create(path).unwrap();
            serde_json::to_writer_pretty(writer, &program).unwrap();
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
