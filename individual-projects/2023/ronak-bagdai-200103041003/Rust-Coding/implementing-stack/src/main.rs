fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_val = stack.pop();
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Cannot add more")
    } else {
        stack.push(item);
        println!("Stack : {:?}", stack);
    }
}

fn size(stack: &mut Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    let n: u32 = n.trim().parse().expect("invalid input");
    n
}
fn main() {
    println!("Let us first create a stack for our use");
    println!("Please mention the size of stack");
    let size_stack = input();

    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n\n ****** Menu ****** \n");
        println!("\n 1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit \n");

        println!("Please enter your choice");
        let choice = input();
        match choice {
            1 => {
                println!("Please enter the value to be pushed");
                let item = input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("The poped value is {:?}", pop(&mut stack).unwrap()),

            3 => println!("The stack is {:?}", stack),

            4 => println!("The size of stack is {:?}", size(&mut stack)),

            5 => break,

            _ => println!("Invalid choice"),
        }

        println!("Do you want to continue? 1 = yes / 0 = no");
        let status = input();
        if status == 1 {
            continue;
        } else {
            println!("Thank you for using our stack");
        }
    }
}
