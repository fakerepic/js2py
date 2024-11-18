# js2py

A JavaScript to Python transpiler implemented in Rust that converts JavaScript code to equivalent Python code.

## Project Structure

This project is organized as a Rust workspace with the following components (crates):

- `js2py_lexer`: Tokenization and lexical analysis of JavaScript source code
- `js2py_parser`: Parser that builds an Abstract Syntax Tree (AST) from JavaScript source code
- `js2py_translator`: Source-to-source translator that converts JavaScript AST to Python code

## Requirements

- Cargo build system

## Usage

The project consists of three main components, each with its own CLI or library interface:

### 1. Lexical Analysis (js2py_lexer)

The lexer tokenizes JavaScript source code into a stream of tokens.

```rust
use js2py_lexer::lexer;

let input = "var x = 10;";
let token_stream = lexer::token_stream(input);

// Print tokens
for token in token_stream.iter() {
    println!("{:?}", token);
}
```

To use the lexer CLI:

```bash
cargo run --bin js2py_lexer input.js
```

This will output the token stream with their types and values.

### 2. Parsing (js2py_parser)

The parser converts JavaScript code into an Abstract Syntax Tree (AST).

```rust
use js2py_parser::Parser;

let source_code = "function add(a, b) { return a + b; }";
let mut parser = Parser::new(source_code);
let ast = parser.parse().unwrap();

// Save AST to JSON file
let writer = std::fs::File::create("output-ast.json").unwrap();
serde_json::to_writer_pretty(writer, &ast).unwrap();
```

To use the parser CLI:

```bash
cargo run --bin js2py_parser input.js
```

This will generate an `input.js-ast.json` file containing the AST representation.

### 3. Translation (js2py_translator)

The translator converts JavaScript AST into Python code.

```rust
use js2py_parser::Parser;
use js2py_translator::Ast2Py;

let source_code = "function add(a, b) { return a + b; }";
let mut parser = Parser::new(source_code);
let ast = parser.parse().unwrap();

// Convert to Python with 2-space indentation
let python_code = Ast2Py::new()
    .with_indent(2)
    .build(&ast)
    .code;

println!("{}", python_code);
```

To use the complete pipeline CLI:

```bash
cargo run --bin js2py_translator input.js
```

This will output the transformed Python code to stdout.
