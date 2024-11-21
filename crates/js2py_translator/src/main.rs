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

    #[test]
    fn test_calculator() {
        let source = r#"
const digitals = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

function isDigital(c) {
    var i = 0;
    var j = 1;
    while (i < digitals.length) {
        if (digitals[i] == c) {
            return true;
        }
        i += 1;
    }
    var k = 3;
    return false;
}

const operators = ['+', '-', '*', '/'];

function isOperator(c) {
    var i = 0;
    while (i < operators.length) {
        if (operators[i] == c) {
            return true;
        }
        i += 1;
    }
    return false;
}

function calculateExpression(s) {
    // 定义运算符的优先级
    const precedence = {
        '+': 10,
        '-': 10,
        '*': 20,
        '/': 20
    };

    // 将中缀表达式转换为后缀表达式（逆波兰表达式）
    function infixToPostfix(expression) {
        const output = [];
        const operatorStack = [];
        var i = 0;
        while (i < expression.length) {
            const c = expression[i];

            if (isDigital(c)) {
                // 处理多位数
                var num = '';
                while (i < expression.length && isDigital(expression[i])) {
                    num = num + expression[i];
                    i += 1;
                }
                output.push(num);
                continue; // 已经移动了 i
            } else if (c == '(') {
                operatorStack.push(c);
            } else if (c == ')') {
                while (operatorStack.length > 0 && operatorStack[operatorStack.length - 1] != '(') {
                    var top = operatorStack.pop();
                    output.push(top);
                }
                if (operatorStack.length > 0 && operatorStack[operatorStack.length - 1] == '(') {
                    operatorStack.pop(); // 弹出 '('
                } else {
                    return null;
                }
            } else if (isOperator(c)) {
                while (
                    operatorStack.length > 0 &&
                    isOperator(operatorStack[operatorStack.length - 1]) &&
                    precedence[operatorStack[operatorStack.length - 1]] >= precedence[c]
                ) {
                    var top = operatorStack.pop();
                    output.push(top);
                }
                operatorStack.push(c);
            } else {
                return null
            }
            i += 1;
        }

        // 弹出剩余的运算符
        while (operatorStack.length > 0) {
            const op = operatorStack.pop();
            if (op == '(' || op == ')') {
                return null
            }
            output.push(op);
        }

        return output;
    }

    // 计算后缀表达式的值
    function evaluatePostfix(postfix) {
        const stack = [];

        var i = 0;
        while (i < postfix.length) {
            const token = postfix[i];
            if (isOperator(token)) {
                if (stack.length < 2) {
                    return null
                }
                var top = stack.pop();
                const b = parseFloat(top);
                top = stack.pop();
                const a = parseFloat(top);
                var result = 0;
                if (token == '+') {
                    result = a + b;
                } else if (token == '-') {
                    result = a - b;
                } else if (token == '*') {
                    result = a * b;
                } else if (token == '/') {
                    if (b == 0) {
                        return null
                    }
                    result = a / b;
                } else {
                    return null
                }
                stack.push(result);
            } else {
                stack.push(token);
            }
            i += 1;
        }

        if (stack.length != 1) {
            return null
        }

        return stack.pop();
    }

    const postfix = infixToPostfix(s);
    return evaluatePostfix(postfix);
}

const expressions = [
    "3+4*2/(1-5)", // 1
    "(2+3)*4", // 20
    "10+2*6", // 22
    "100*(2+12)/14" // 100
];

var i = 0;
while (i < expressions.length) {
    const expr = expressions[i];
    const result = calculateExpression(expr);
    if (result == null) {
        console.log('failed to calculate the expression');
        i += 1;
        continue;
    }
    console.log('result: ');
    console.log(result);
    i += 1;
}
            "#;
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();

        let test_name = "test_calculator";
        let path = format!("{}.ast.json", test_name);
        let writer = std::fs::File::create(path).unwrap();
        serde_json::to_writer_pretty(writer, &ast).unwrap();

        let python_code = Ast2Py::new().with_indent(4).build(&ast).code;
        println!("{}", python_code);
        let output_path = format!("{}.py", test_name);
        std::fs::write(output_path, python_code).unwrap();
    }
}
