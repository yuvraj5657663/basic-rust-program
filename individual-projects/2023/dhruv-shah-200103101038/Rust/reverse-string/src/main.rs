// reverce string
// String Reversal Using Stacks

fn _input() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: u32 = input.trim().parse().expect("Invalid input");
    input
}

fn stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let pop = stack.pop();
    pop
}

fn push(stack: &mut Vec<char>, value: char, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(value);
    }
}

fn size(stack: &mut Vec<char>) -> usize {
    stack.len()
}

fn main() {
    let _input_string = String::from("welcome to rust");
    let size_stack = _input_string.len();
    let mut stack = stack(size_stack);

    // rev string new
    let mut rev_string = String::new();

    for i in _input_string.chars() {
        push(&mut stack, i, size_stack);
    }
    for _i in 0..size(&mut stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("the input string is {:?}", _input_string);
    println!("the reverse string is {:?}", rev_string);
}

//  we use the chars method to convert the input string into an iterator of characters.
//  We then call the rev method on the iterator to reverse the order of the characters.
//  Finally, we use the collect method to collect the reversed characters into a new String.

// fn rev_string(input: &str) -> String {
//     input.chars().rev().collect()
// }

// fn main() {
//     let input = "hello";
//     let rev = rev_string(input);
//     println!("string is : {}", input);
//     println!("rev string is : {}", rev);
// }
