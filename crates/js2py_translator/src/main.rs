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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insertion_sort() {
        let source = r#"
function insertionSort(arr) {
    let i = 1;
    while (i < arr.length) {
        const key = arr[i];
        let j = i - 1;
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
        i = i + 1;
    }
    return arr;
}
let array = [5.3, 2.1, 8.7, 1.9, 3.4];
console.log(insertionSort(array));
            "#;
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();

        let test_name = "test_insertion_sort";
        let path = format!("{}.ast.json", test_name);
        let writer = std::fs::File::create(path).unwrap();
        serde_json::to_writer_pretty(writer, &ast).unwrap();

        let python_code = Ast2Py::new().with_indent(4).build(&ast).code;
        println!("{}", python_code);
        let output_path = format!("{}.py", test_name);
        std::fs::write(output_path, python_code).unwrap();
    }
}
