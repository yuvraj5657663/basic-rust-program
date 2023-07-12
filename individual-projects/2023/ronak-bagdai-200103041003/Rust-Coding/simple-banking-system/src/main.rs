// Simple Banking System in Rust

use serde_json;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

fn main() {
    let mut balance = load_balance();

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
            save_balance(balance);
            println!("Thank you for using the Rust Banking System. GoodBye!");
            println!("Your current balance is: ${:.2}", balance);
            break;
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
        println!("Withdrawal Successful. New Balance is: ${:.2}", balance);
    } else if amount > *balance {
        println!(
            "Insufficient Funds. Your current balance is: ${:.2}",
            balance
        );
    } else {
        println!("Invalid amount. Please enter a positive number.");
    }
}

fn load_balance() -> f64 {
    let mut file = File::open("balance.json").expect("failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("failed to read");
    let data = serde_json::from_str::<serde_json::Value>(&content).expect("failed to parse json");
    let balance = data["balance"].as_f64().expect("balance value not found");
    balance
}

fn save_balance(balance: f64) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("balance.json")
        .expect("failed to open file");

    let data = serde_json::json!({ "balance": balance });

    let balance_str = serde_json::to_string_pretty(&data).expect("failed to serialize json");
    file.write(balance_str.as_bytes()).expect("failed to write");
}
