fn main() {
    let mut balance = 10000.00;

    loop {
        println!("***** Welcome to Rust Banking System *****");
        println!("1. Check Balance");
        println!("2. Deposit Money");
        println!("3. Withdraw Money");
        println!("4. Exit\n");
        println!("Enter your Choice:");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = choice.trim().parse().expect("invalid input");

        if choice == 1 {
            check_balance(&balance);
        } else if choice == 2 {
            deposit_money(&mut balance);
        } else if choice == 3 {
            withdraw_money(&mut balance);
        } else if choice == 4 {
            println!("Thank you for using the Rust Banking System. GoodBye!");
            println!("Your current balance is: ${:.2}", balance);
        } else {
            println!("Invalid choice. Please enter a number from 1 to 4.");
        }
    }
}

fn check_balance(balance: &f64) {
    println!("Your current balance is: ${:.2}", balance);
}

fn deposit_money(balance: &mut f64) {
    println!("Enter the amount to deposit:");
    let mut amount = String::new();
    std::io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    let amount: f64 = amount.trim().parse().expect("invalid input");

    if amount > 0.0 {
        *balance = *balance + amount;
        println!("Deposit Successful. New Balance is: ${:.2}", balance);
    } else {
        println!("Invalid Amount. Please enter a positive number.");
    }
}

fn withdraw_money(balance: &mut f64) {
    println!("Enter the amount to withdraw:");
    let mut amount = String::new();
    std::io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    let amount: f64 = amount.trim().parse().expect("invalid input");

    if amount > 0.0 && amount <= *balance {
        *balance = *balance - amount;
    } else if amount > *balance {
        println!(
            "Insufficient Funds. Your current balance is: ${:.2}",
            balance
        );
    } else {
        println!("Invalid amount. Please enter a positive number.");
    }
}
