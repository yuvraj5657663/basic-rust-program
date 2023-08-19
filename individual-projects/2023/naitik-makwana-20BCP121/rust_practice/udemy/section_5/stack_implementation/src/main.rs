// Project : Stack implementation

fn main() {
    println!("* creating stack *");
    println!("Enter the size of the stack");

    let ms = take_input();

    let mut stack:Vec<u32> =  new_stack(ms as usize);


    loop {

        println!("### Task can be performed on stack ###");
        println!("1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit ");

        println!("Enter choice : ");
        let choice:u32 = take_input();

        match choice {
            1 => {
                println!("Enter value to insert : ");
                let item = take_input();
                push(&mut stack, item, ms as usize);
            }

            2 => println!("the poped element {:?}",pop(&mut stack)),
            3 => println!("the elements are {:?}", stack ),
            4 => println!("the size of stack is {}", size(&stack)),
            5 => break,
            _ => println!("Wrong input try again"),
        }
        
        println!("Wanna continue : press 1 for yes, 0 for no", );

        let status = take_input();
        if status == 1{
            continue;
        }else {
            break;
        }
    }

}

fn take_input() -> u32 {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed");

    let n:u32 =  n.trim().parse().unwrap();
    n
}

fn new_stack(ms : usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(ms);
    vec
}

fn pop(stack : &mut Vec<u32>) -> Option<u32> {
    let pop_val :Option<u32> =  stack.pop();
    println!("the poped value is {:?}", pop_val);
    pop_val
}

fn push(stack : &mut Vec<u32>, input:u32, ms:usize) {
    if stack.len() == ms{
        println!("can't add more");
    }else {
        stack.push(input);
        println!("{:?}",stack );
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}