use js2py_parser::Parser;
use js2py_translator::Ast2Py;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let source_code = std::fs::read_to_string(filename).unwrap();

    let mut parser = Parser::new(&source_code);
    let ast = parser.parse().unwrap();

    let python_code = Ast2Py::default().build(&ast).code;

    let output_path = format!("{}.py", filename);
    std::fs::write(output_path, python_code).unwrap();

    println!("wrote python code into {}.py", filename)
}
