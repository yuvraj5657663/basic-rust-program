fn priority(x: &String) -> u8 {
    if ("+" == x) || ("-" == x) {
        return 1;
    } else if ("*" == x) || ("/" == x) {
        return 2;
    } else if "^" == x {
        return 3;
    } else {
        return 0;
    }
}

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let poped_val: Option<String> = stack.pop();
    poped_val
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
        // println!("Cannot add more")
    } else {
        stack.push(item);
        // println!("Stack : {:?}", stack);
    }
}

fn size(stack: &mut Vec<String>) -> usize {
    stack.len()
}

fn individual_expression(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_chars: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();
    for i in input_chars {
        if i != '+' && i != '-' && i != '*' && i != '/' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }
        }
    }

    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("Tokenized Input : {:?}", tokenized_input);
    tokenized_input
}

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expr = input.len();
    let mut stack: Vec<String> = new_stack(size_expr);
    let mut postfix: Vec<String> = Vec::new();
    for i in input {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
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

    println!("Postfix Expression : {:?}", postfix);
    postfix
}

fn operation(oper: String, op1: String, op2: String) -> f32 {
    let op1: f32 = op1.parse().unwrap();
    let op2: f32 = op2.parse().unwrap();
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

fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    let size_expr = postfix.len();
    let mut result_stack: Vec<String> = new_stack(size_expr);
    for i in postfix {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                let oper = i;
                let op2 = pop(&mut result_stack).unwrap();
                let op1 = pop(&mut result_stack).unwrap();
                let result = operation(oper, op1, op2);
                push(&mut result_stack, result.to_string(), size_expr);
            }
            _ => push(&mut result_stack, i.to_string(), size_expr),
        }
    }
    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}

fn main() {
    let input_expr = String::from("(33+45/3*(2+9)-50)");
    println!("Input Expression : {:?}", input_expr);
    let tokenized_input = individual_expression(input_expr);

    let postfix = infix_to_postfix(tokenized_input);

    println!("Result : {:?}", postfix_evaluation(postfix));
}
