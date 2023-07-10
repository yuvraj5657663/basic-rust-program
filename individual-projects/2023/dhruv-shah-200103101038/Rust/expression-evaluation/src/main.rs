// expresion evaluation

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let pop_value = stack.pop();
    pop_value
}

fn push(stack: &mut Vec<String>, value: String, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(value);
    }
}

fn size(stack: &mut Vec<String>) -> usize {
    stack.len()
}

fn individual_symbol(input_expr: String) -> Vec<String> {
    let mut token_input: Vec<String> = Vec::new();
    let input_char: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();
    for i in input_char {
        if i != '+' && i != '-' && i != '*' && i != '/' && i != '(' && i != ')' && i != '^' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                token_input.push(i.to_string());
            } else {
                token_input.push(temp.into_iter().collect());
                token_input.push(i.to_string());
                temp = vec![]
            }
        }
    }
    if temp.len() != 0 {
        token_input.push(temp.iter().collect());
    }
    println!("{:?}", token_input);
    token_input
}
//

// infix -> postfix expression
fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expr = input.len();
    let mut stack = new_stack(size_expr);
    let mut postfix: Vec<String> = Vec::new();
    for i in input {
        match i.as_str() {
            "+" | "-" | "/" | "*" | "^" => {
                if size(&mut stack) == 0 {
                    push(&mut stack, i, size_expr);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        push(&mut stack, i, size_expr);
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            postfix.push(pop(&mut stack).unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }
                        push(&mut stack, i, size_expr);
                    }
                }
            }

            "(" => push(&mut stack, i, size_expr),
            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap();
            }

            _ => postfix.push(i),
        }
    }

    while size(&mut stack) != 0 {
        postfix.push(pop(&mut stack).unwrap());
    }
    println!("{:?}", postfix);

    postfix
}

fn priority(x: &String) -> u8 {
    if ("+" == x) | ("-" == x) {
        1
    } else if ("*" == x) | ("/" == x) {
        2
    } else if "^" == x {
        3
    } else {
        0
    }
}
//

// postfix evaluation
fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    let size_expr = postfix.len();
    let mut result_stack = new_stack(size_expr);
    for i in postfix {
        match i.as_str() {
            "+" | "-" | "*" | "/" => {
                let oper = i;
                let op2 = pop(&mut result_stack).unwrap();
                let op1 = pop(&mut result_stack).unwrap();
                let result = operation(op1, op2, oper);

                push(&mut result_stack, result.to_string(), size_expr);
            }
            _ => push(&mut result_stack, i.to_string(), size_expr),
        }
    }
    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}
fn operation(op1: String, op2: String, oper: String) -> f32 {
    let op1 = op1.parse::<f32>().unwrap();
    let op2 = op2.parse::<f32>().unwrap();
    let result = match oper.as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _ => 0.0,
    };
    result
}
//
fn main() {
    let string = String::from("(33+45/3*(2+9)-50)");
    println!("the expresion is {:?}", string);

    let _input_expr_token: Vec<String> = individual_symbol(string);

    let postfix = infix_to_postfix(_input_expr_token);
    println!("the postfix expresion is {:?}", postfix_evaluation(postfix));
}
