mod ast2py;
pub use ast2py::*;

pub fn make_indent(s: &str, indent: usize) -> String {
    s.split("\n")
        .map(|line| format!("{:indent$}{}", "", line, indent = indent))
        .collect::<Vec<_>>()
        .join("\n")
}
