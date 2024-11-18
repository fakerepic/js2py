use js2py_parser::Parser;
use js2py_translator::Ast2Py;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }

    let source_code = std::fs::read_to_string(&args[1]).unwrap();

    let mut parser = Parser::new(&source_code);
    let ast = parser.parse().unwrap();

    let python_code = Ast2Py::default().build(&ast).code;

    println!("{}", python_code);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let source = r#"
    function add(a, b) {
    function y() {
        return a + b;
    }
    return y;
    }
    function multi(a, b) {
    return a * b;
    }
    let f = 1 + 2;
            "#;
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();

        let python_code = Ast2Py::new().with_indent(2).build(&ast).code;
        println!("{}", python_code);
    }
}
