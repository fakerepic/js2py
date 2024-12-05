mod ast2py;
pub use ast2py::*;

pub fn make_indent(s: &str, indent: usize) -> String {
    s.split("\n")
        .map(|line| format!("{:indent$}{}", "", line, indent = indent))
        .collect::<Vec<_>>()
        .join("\n")
}

pub trait PlaceHolder {
    fn with_placeholder(self, placeholder: &str) -> String;
}
impl PlaceHolder for String {
    fn with_placeholder(self, placeholder: &str) -> String {
        if self.is_empty() {
            placeholder.to_string()
        } else {
            self
        }
    }
}
