// Simple Banking System in Rust

use std::fs::{File, OpenOptions};
use std::io::prelude::*;

fn main() {
    let mut balance = load_balance();
    let (mut notes_2000, mut notes_500, mut atm_balance) = load_atm_notes();

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
            check_balance(balance);
        } else if choice == 2 {
            deposit_money(
                &mut balance,
                &mut notes_2000,
                &mut notes_500,
                &mut atm_balance,
            );
        } else if choice == 3 {
            withdraw_money(
                &mut balance,
                &mut notes_2000,
                &mut notes_500,
                &mut atm_balance,
            );
        } else if choice == 4 {
            save_balance(balance);
            save_atm_notes(notes_2000, notes_500, balance);
            println!("Thank you for using the Rust Banking System. GoodBye!");
            println!("Your current balance is: ${:.2}", balance);
            break;
        } else {
            println!("Invalid choice. Please enter a number from 1 to 4.");
        }
    }
}

fn check_balance(balance: f64) {
    println!("Your current balance is: ${:.2}", balance);
}

fn deposit_money(
    balance: &mut f64,
    notes_2000: &mut u64,
    notes_500: &mut u64,
    atm_balance: &mut f64,
) {
    println!("Enter the amount to deposit:");
    let mut amount = String::new();
    std::io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    let _amount: f64 = amount.trim().parse().expect("Invalid input");

    println!("Enter the number of 2000 Rs notes:");
    let mut num_2000_notes = String::new();
    std::io::stdin()
        .read_line(&mut num_2000_notes)
        .expect("Failed to read input");
    let num_2000_notes: u64 = num_2000_notes.trim().parse().expect("Invalid input");

    println!("Enter the number of 500 Rs notes:");
    let mut num_500_notes = String::new();
    std::io::stdin()
        .read_line(&mut num_500_notes)
        .expect("Failed to read input");
    let num_500_notes: u64 = num_500_notes.trim().parse().expect("Invalid input");

    let total_deposit = (num_2000_notes * 2000 + num_500_notes * 500) as f64;

    *balance += total_deposit;
    *notes_2000 += num_2000_notes;
    *notes_500 += num_500_notes;
    *atm_balance += total_deposit;

    println!("Deposit Successful. New Balance is: ${:.2}", balance);
}

fn withdraw_money(
    balance: &mut f64,
    notes_2000: &mut u64,
    notes_500: &mut u64,
    atm_balance: &mut f64,
) {
    println!("Enter the amount to withdraw:");
    let mut amount = String::new();
    std::io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    let amount: f64 = amount.trim().parse().expect("Invalid input");

    if amount <= 0.0 {
        println!("Invalid amount. Please enter a positive number.");
    } else {
        let mut remaining_amount = amount;

        let withdraw_2000_notes = std::cmp::min(remaining_amount as u64 / 2000, *notes_2000);
        remaining_amount -= withdraw_2000_notes as f64 * 2000.0;
        *notes_2000 -= withdraw_2000_notes;

        let withdraw_500_notes = std::cmp::min(remaining_amount as u64 / 500, *notes_500);
        remaining_amount -= withdraw_500_notes as f64 * 500.0;
        *notes_500 -= withdraw_500_notes;

        if remaining_amount > 0.0 {
            println!("Unable to withdraw amount. Please enter amount in multiples of 500.");
        } else {
            *balance -= amount;
            *atm_balance -= amount;
            println!("Withdraw Successful. New Balance is: ${:.2}", balance);
        }

        println!("Number of 2000 Rs notes withdrawn: {}", withdraw_2000_notes);
        println!("Number of 500 Rs notes withdrawn: {}", withdraw_500_notes);
    }
}

fn load_balance() -> f64 {
    let mut file = File::open("user_balance.json").expect("failed to open file");
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
        .open("user_balance.json")
        .expect("failed to open file");

    let data = serde_json::json!({ "balance": balance });

    let balance_str = serde_json::to_string_pretty(&data).expect("failed to serialize json");
    file.write(balance_str.as_bytes()).expect("failed to write");
}

fn load_atm_notes() -> (u64, u64, f64) {
    let mut file = File::open("atm.json").expect("Failed to open ATM file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read ATM file");

    let data =
        serde_json::from_str::<serde_json::Value>(&content).expect("Failed to parse ATM JSON");
    let notes_2000 = data["notes_2000"]
        .as_u64()
        .expect("notes_2000 value not found");
    let notes_500 = data["notes_500"]
        .as_u64()
        .expect("notes_500 value not found");
    let atm_balance = data["balance"].as_f64().expect("balance value not found");
    (notes_2000, notes_500, atm_balance)
}

fn save_atm_notes(notes_2000: u64, notes_500: u64, atm_balance: f64) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("atm.json")
        .expect("Failed to open ATM file");

    let data = serde_json::json!({
        "notes_2000": notes_2000,
        "notes_500": notes_500,
        "balance": atm_balance
    });

    let atm_notes_str = serde_json::to_string_pretty(&data).expect("Failed to serialize ATM JSON");

    file.write_all(atm_notes_str.as_bytes())
        .expect("Failed to write ATM file");
}
