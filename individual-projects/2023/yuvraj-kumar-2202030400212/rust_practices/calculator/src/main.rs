use std :: io;
fn main(){
    println!("simple calculator");
    loop{
        println!("select an operation");
        println!("1. add");
        println!("2. sub");
        println!("3. mul");
        println!("4. div");
        println!("5. square root");
        println!("0. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");

        let choice: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. please enter a valid number.");
                continue;
            }

        };
        
        if choice == 0 {
            break;
        }
        match choice{
            1|2|3|4 => {
                println!("Enter the first number:");
                let num1: f64 = read_number();
                
                println!("Enter the second number:");
                let num2: f64 = read_number();

                match choice{
                    1 => println!("result:{}",num1+num2),
                    2 => println!("result:{}",num1-num2),
                    3 => println!("result:{}", num1*num2),
                    4 => {
                        if num2 != 0.0{
                            println!("reault: {}",num1/num2);

                        }else{
                            println!("Error:Division by zero is not allowed.");
                        }
                    }
                    _=> {}
                }
            }
            5 =>{
                println!("enter the number:");
                let num: f64 = read_number();
                if num >= 0.0{
                    println!("Square Root: {}", num.sqrt());
                }else{
                    println!("Error: Square root of a negative number is not defined.");
                }
            }
            _=>{
                println!("Invalid choice. Please enter a valid option.");
            }
        }
    }
    println!("goodbye!");
}
fn read_number() -> f64{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    match input.trim().parse(){
        Ok(num)=>num,
        Err(_) => {
            println!("Invalid input. please enter a valid number.");
            read_number()
        }
    }
}
