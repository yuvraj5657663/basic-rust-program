const PIN: i32 = 1234;

fn user_input() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("enter number");
    let x: i32 = input.trim().parse().expect("failed");
    x
}

fn check_balance(balance: i32) {
    println!("Your balance is {}", balance)
}

fn deposit(balance: &mut i32) {
    println!("Enter amount to deposit");
    let amount = user_input();
    if amount > 0 {
        *balance += amount;
        println!(
            "Deposit successful! Amount deposited: {}. Updated balance: {}",
            amount, balance
        );
    } else {
        println!("Invalid amount");
    }
}

fn withdraw(balance: &mut i32) {
    println!("Enter amount to withdraw");
    let amount = user_input();
    if amount > 0 && amount <= *balance {
        *balance -= amount;
        println!(
            "Withdrawal successful! Amount withdrawn: {}. Updated balance: {}",
            amount, balance
        );
    } else {
        println!("Insufficient funds");
    }
}

fn main() {
    let mut balance = 100000;
    println!("Enter your pin");
    let user_pin = user_input();

    if user_pin == PIN {
        loop {
            println!("please select an option");
            println!("\n 1. Check balance \n 2. Withdraw \n 3. Deposit \n 4. Exit");

            let choice = user_input();

            match choice {
                1 => {
                    check_balance(balance);
                }
                2 => {
                    withdraw(&mut balance);
                }
                3 => {
                    deposit(&mut balance);
                }
                4 => {
                    println!("Thank you for using our ATM");
                    break;
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    } else {
        println!("Invalid pin");
    }
}
