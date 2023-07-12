use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const PIN: u32 = 1234;

// This function loading the account balance from the "account.json" file.
fn load_account_balance() -> f32 {
    let mut file = File::open("account.json").expect("failed to read.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read");
   
    let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON.");
    let balance = json_data["balance"].as_f64().unwrap_or(0.0) as f32;
    balance
}

// this function saves the account balance to the "account.json" file.
fn save_account_balance(balance: f32) {
    let json_data = json!({
        "balance": balance,
    });

    let json_str = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON.");
    let mut file = File::create("account.json").expect("Failed to create file.");

    file.write_all(json_str.as_bytes())
        .expect("Failed to write to file.");
}

fn user_input() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("enter number");
    let x: u32 = input.trim().parse().expect("failed");
    x
}

fn user_input_amount() -> f32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("enter number");
    let x: f32 = input.trim().parse().expect("failed");
    x
}

fn check_balance(balance: f32) {
    println!("Your balance is {}", balance)
}

fn deposit(balance: &mut f32) {
    println!("Enter amount to deposit");
    let amount = user_input_amount();
    *balance += amount;
    println!(
        "Deposit successful! Amount deposited: {}. Updated balance: {}",
        amount, balance
    );
}

fn withdraw(balance: &mut f32) {
    println!("Enter amount to withdraw");
    let amount = user_input_amount();
    if amount > 0.0 && amount <= *balance {
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
    let mut balance = load_account_balance();

    println!("Enter your pin");
    let user_pin: u32 = user_input();

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
                    save_account_balance(balance);
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
