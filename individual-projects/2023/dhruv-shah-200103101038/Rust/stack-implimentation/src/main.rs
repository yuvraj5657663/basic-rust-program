// stack implimentation
// short trick ---- LIFO(Last In First Out)

fn input() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: u32 = input.trim().parse().expect("Invalid input");
    input
}

fn stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> u32 {
    let pop = stack.pop().expect("Stack is empty");
    pop
}

fn push(stack: &mut Vec<u32>, value: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Stack is full");
    } else {
        stack.push(value);
    }
}

fn size(stack: &mut Vec<u32>) -> usize {
    stack.len()
}

fn main() {
    println!("Enter the size of stack");

    let maxsize = input();

    let mut stack: Vec<u32> = stack(maxsize as usize);

    loop {
        println!("1. Push \n2. Pop \n3. Display \n4. Size \n5. Exit");

        let choice = input();
        println!("entered choice is:");

        match choice {
            1 => {
                println!("Enter the value to push");
                let value = input();
                push(&mut stack, value, maxsize as usize);
            }
            2 => {
                let pop = pop(&mut stack);
                println!("Popped value is {}", pop);
            }
            3 => {
                println!("Stack is {:?}", stack);
            }
            4 => {
                let size = size(&mut stack);
                println!("Size of stack is {}", size);
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
        println!("do you continue : 1.yes 2.no");
        let status = input();
        if status == 2 {
            break;
        }
    }
}
