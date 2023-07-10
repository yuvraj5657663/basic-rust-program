fn main() {
    println!("******Simple Calculator*****");
    loop {
        println!("Enter the first number");
        let mut num1 = String::new();
        std::io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read input");
        let num1: f32 = num1.trim().parse().expect("invalid input");

        println!("Enter the second number");
        let mut num2 = String::new();
        std::io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read input");
        let num2: f32 = num2.trim().parse().expect("invalid input");

        println!("Enter the operation to be performed (+, -, *, /, %, s)");
        let mut operation = String::new();
        std::io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read input");
        let operation: char = operation.trim().parse().expect("invalid input");

        match operation {
            '+' => println!("The addition of {} and {} is {}", num1, num2, num1 + num2),
            '-' => println!(
                "The subtraction of {} and {} is {}",
                num1,
                num2,
                num1 - num2
            ),
            '*' => println!(
                "The multiplication of {} and {} is {}",
                num1,
                num2,
                num1 * num2
            ),
            '/' => println!("The division of {} and {} is {}", num1, num2, num1 / num2),
            '%' => println!("The remainder of {} and {} is {}", num1, num2, num1 % num2),
            's' => println!(
                "The square root of {} and {} are {} and {}",
                num1,
                num2,
                num1.sqrt(),
                num2.sqrt()
            ),
            _ => println!("Invalid operation"),
        }

        println!("Do you want to continue? (y/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: char = choice.trim().parse().expect("invalid input");

        if choice != 'y' {
            println!("Thank you for using the calculator");
            break;
        }
    }
}
