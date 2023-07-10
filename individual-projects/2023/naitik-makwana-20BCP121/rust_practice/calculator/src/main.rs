/*
Project : Simple calculator

Problem Statement : Create a simple calculator program in Rust that supports basic arithmetic operations as well as advanced functions such as square root. Do not create any UI. Add multiple examples in console to validate code.

*/

fn addition(op1:i32, op2:i32) -> i32 {
    op1 + op2
}

fn subtraction(op1:i32, op2:i32) -> i32 {
    op1 - op2
}

fn multiplication(op1:i32, op2:i32) -> i32 {
    op1 * op2
}

fn division(op1:i32, op2:i32) -> i32 {
    op1 / op2
}

fn modulus (op1:i32, op2:i32) -> i32 {
    op1 % op2
}

fn power(op1:i32, op2:i32) -> i32 {
    op1.pow(op2 as u32)
}

fn square_root(op1:i32) -> f64 {
    (op1 as f64).sqrt()
} 

fn take_input () -> i32 {

    let mut op = String::new();

    std::io::stdin().read_line(&mut op).expect("please Enter numeric value");
    let op = op.trim().parse::<i32>().expect("Unable to read input");
    op
} 

fn main () {

    println!("##### Calculator #####");

    let mut operation_selection = 0;
    let mut op1 = 0;
    let mut op2 = 0;
    let mut wanna_continue = 0;

    loop {
        println!("Press number accordingly to select any operation : \n 1. Addition (+) \n 2. Subtraction (-) \n 3. Multiplication (*) \n 4. Division (/) \n 5. Modulus (%) \n 6. Power (^) \n 7. Square Root \n 8. Exit " );
        println!("Enter a number to select operation :");
    
        //input to select operation 
        operation_selection = take_input();
    
        if operation_selection >= 1 && operation_selection<=6 {
            println!("Enter first operand : ");
            op1 = take_input();
            println!("Enter second operand : ");
            op2 = take_input();

            match operation_selection {
                1 => println!("Your answer = {}",addition(op1,op2)),
                2 => println!("Your answer = {}",subtraction(op1,op2)),
                3 => println!("Your answer = {}",multiplication(op1,op2)),
                4 => println!("Your answer = {}",division(op1,op2)),
                5 => println!("Your answer = {}",modulus(op1,op2)),
                6 => println!("Your answer = {}",power(op1,op2)),
                _ => println!("Something went wrong"),

            }

        }else if operation_selection == 7 {
            println!("Enter a operand");
            op1 = take_input();
            println!("Your answer = {}",square_root(op1));

        }else if operation_selection == 8 {
            println!("Thank you for using calculator.");
            break;
        }else {
            println!("Unexpected input!!");
        }

        println!("Do you want to continue using calculator : \n 1. Yes \n 2. No");
        wanna_continue = take_input();

        if wanna_continue==2 {
            println!("Thank you for using calculator.");
            break;
        }

    }
    
}