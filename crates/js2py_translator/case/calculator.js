const digitals = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

function isDigital(c) {
    var i = 0;
    while (i < digitals.length) {
        if (digitals[i] == c) {
            return true;
        }
        i += 1;
    }
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
                // 处理负数
                if (c == '-' && (i == 0 || !isDigital(expression[i - 1]) && expression[i - 1] != ')')) {
                    var num = '-';
                    i += 1;
                    while (i < expression.length && isDigital(expression[i])) {
                        num = num + expression[i];
                        i += 1;
                    }
                    output.push(num);
                    continue; // 已经移动了 i
                }
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
    "1/2", // 0.5
    "-2+1", // -1
    "1+(-5-22)*4/(2+1)", // -35
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
