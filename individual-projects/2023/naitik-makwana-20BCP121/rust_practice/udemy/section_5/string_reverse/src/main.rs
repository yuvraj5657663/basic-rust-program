// String reversal

fn main() {

    let in_str = take_input();
    let ms = in_str.len();
    let mut stack:Vec<char> = new_stack(ms);
    let mut rev_str = String::new();

    for i in in_str.chars() {
        push (&mut stack, i, ms);
    }

    for i in 0..size(&stack) {
        rev_str.push(pop(&mut stack).unwrap());
    }

    println!("The input string : {:?}", in_str );
    println!("The output string : {:?}", rev_str);

}

fn take_input() -> String {

    println!("Enter a string : ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed");

    let n:String =  n.trim().to_string();
    n
}

fn new_stack(ms : usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(ms);
    vec
}

fn pop(stack : &mut Vec<char>) -> Option<char> {
    let pop_val :Option<char> =  stack.pop();
    pop_val
}

fn push(stack : &mut Vec<char>, input:char, ms:usize) {
    if stack.len() == ms{
    }else {
        stack.push(input);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}