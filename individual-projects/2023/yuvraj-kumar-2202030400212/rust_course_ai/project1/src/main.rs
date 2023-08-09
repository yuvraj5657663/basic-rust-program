use std::io;

fn main(){
    println!("Simple calculator");

    println!("Enter the first number: ");
    let mut num1 = String::new ();
    io::stdin().read_line(&mut num1).expect("failed to read line");
    let num1: f64 = num1.trim().parse().expect("Invalid input");

    println!("enter the second number: ");
    let mut num2 =String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("invalid input");

    println!("Select an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. multiply");
    println!("4. divide");

    let mut choice = String::new ();
    io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("invalid input");

    match choice {
        1 => println!("result: {}", num1 + num2),
        2 => println!("Result: {}", num1 - num2), 
        3 => println!("result: {}", num1 * num2),
        4 => {
            if num2 !=0.0{
                println!("result: {}", num1/num2);

            }else{
                println!("cannot divide by zero");

            }
        },
        _=> println!("invalid choice"), 

    }
}