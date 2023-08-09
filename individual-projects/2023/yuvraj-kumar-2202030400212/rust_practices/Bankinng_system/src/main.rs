use std::io;
fn main() {
    let mut balance = 1000.0; // current balance

    loop {
        println!("Select an option:");
        println!("1. Check Balance");
        println!("2. Deposit Money");
        println!("3. Withdraw Money");
        println!("4. Exit");

        let choice: u32 = get_input("Enter choice:").parse().expect("Invalid input");

        match choice {
            1 => println!("Your current balance is: {}", balance),
            2 => {
                let amount: f64 = get_input("Enter amount to deposit:").parse().expect("Invalid amount");
                if amount > 0.0 {
                    balance += amount;
                    println!("Deposited {} successfully. New balance: {}", amount, balance);
                } else {
                    println!("Invalid deposit amount.");
                }
            }
            3 => {
                let amount: f64 = get_input("Enter amount to withdraw:").parse().expect("Invalid amount");
                if amount > 0.0 && balance >= amount {
                    balance -= amount;
                    println!("Withdrawn {} successfully. New balance: {}", amount, balance);
                } else if amount <= 0.0 {
                    println!("Invalid withdrawal amount.");
                } else {
                    println!("Insufficient balance.");
                }
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
