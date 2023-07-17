use serde_json::{json, Value};
use std::fs::File;
use std::io::{Read, Write};

const PIN: u32 = 1234;

fn load_account_balance() -> f32 {
    let mut file = File::open("account.json").expect("failed to read.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read");

    let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON.");
    let balance = json_data["balance"].as_f64().unwrap_or(0.0) as f32;
    balance
}

fn save_account_balance(balance: f32) {
    let json_data = json!({
        "balance": balance,
    });

    let json_str = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON.");
    let mut file = File::create("account.json").expect("Failed to create file.");

    file.write_all(json_str.as_bytes())
        .expect("Failed to write to file.");
}

fn load_atm_notes() -> (u32, u32, f32) {
    let mut file = File::open("atm.json").expect("failed to read.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read");

    let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON.");
    let notes_500 = json_data["five_hundred_note"].as_u64().unwrap_or(0) as u32;
    let notes_100 = json_data["one_hundred_note"].as_u64().unwrap_or(0) as u32;
    let atm_balance = json_data["atm_balance"].as_f64().unwrap_or(0.0) as f32;
    (notes_500, notes_100, atm_balance)
}

fn save_atm_notes(notes_500: u32, notes_100: u32, atm_balance: f32) {
    let json_data = json!({
        "atm_balance": atm_balance,
        "five_hundred_note": notes_500,
        "one_hundred_note": notes_100,
    });

    let json_str = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON.");
    let mut file = File::create("atm.json").expect("Failed to create file.");

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

fn deposit(balance: &mut f32, notes_500: &mut u32, notes_100: &mut u32, atm_balance: &mut f32) {
    println!("Enter amount to deposit");
    let amount = user_input_amount();

    println!("enter the 500 notes");
    let num_notes_500 = user_input();

    println!("enter the 100 notes");
    let num_notes_100 = user_input();

    let total_amount = num_notes_500 as f32 * 500.0 + num_notes_100 as f32 * 100.0;

    *balance += total_amount; // update account balance
    *notes_500 += num_notes_500;
    *notes_100 += num_notes_100;
    *atm_balance += total_amount; // update atm balance

    println!(
        "Deposit successful! Amount deposited: {}. Updated balance: {}",
        amount, *balance
    );

    save_account_balance(*balance);
    save_atm_notes(*notes_500, *notes_100, *atm_balance);
}

fn withdraw(balance: &mut f32, notes_500: &mut u32, notes_100: &mut u32, atm_balance: &mut f32) {
    println!("Enter amount to withdraw");
    let amount = user_input_amount();

    if amount > *balance {
        println!("Insufficient funds");
        return;
    }

    let mut _remaining_amount = amount;

    let withdrawn_notes_500 = std::cmp::min(_remaining_amount as u32 / 500, *notes_500);
    _remaining_amount -= withdrawn_notes_500 as f32 * 500.0;
    *notes_500 -= withdrawn_notes_500;

    let withdrawn_notes_100 = std::cmp::min(_remaining_amount as u32 / 100, *notes_100);
    _remaining_amount -= withdrawn_notes_100 as f32 * 100.0;
    *notes_100 -= withdrawn_notes_100;

    *balance -= amount;
    *atm_balance -= amount;

    println!(
        "Withdrawal successful! Amount withdrawn: {}. Updated balance: {}",
        amount, *balance
    );

    println!("Number of 500 notes withdrawn: {}", withdrawn_notes_500);
    println!("Number of 100 notes withdrawn: {}", withdrawn_notes_100);

    save_account_balance(*balance);
    save_atm_notes(*notes_500, *notes_100, *atm_balance);
}

fn main() {
    println!("Welcome to ATM");
    let mut balance = load_account_balance();
    let (mut notes_500, mut notes_100, mut atm_balance) = load_atm_notes();

    println!("Enter your pin");
    let user_pin: u32 = user_input();

    if user_pin == PIN {
        loop {
            println!("please select an option");
            println!("\n 1. Check balance \n 2. Withdraw \n 3. Deposit \n 4. Exit");

            let choice = user_input();

            match choice {
                1 => check_balance(balance),
                2 => {
                    withdraw(
                        &mut balance,
                        &mut notes_500,
                        &mut notes_100,
                        &mut atm_balance,
                    );
                }
                3 => {
                    deposit(
                        &mut balance,
                        &mut notes_500,
                        &mut notes_100,
                        &mut atm_balance,
                    );
                }
                4 => {
                    println!("Thank you for using our ATM");
                    save_account_balance(balance);
                    save_atm_notes(notes_500, notes_100, atm_balance); // Update with atm_balance
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
