use std::fs::File;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};
use serde_json;

const ACCOUNT_NUMBER: u32 = 123456;
const PIN: u32 = 1234;
const HUNDRED_NOTE_VALUE: f32 = 100.0;
const TWO_THOUSAND_NOTE_VALUE: f32 = 2000.0;

#[derive(Serialize, Deserialize)]
struct NotesData {
    hundred_notes: u32,
    two_thousand_notes: u32,
}

#[derive(Serialize, Deserialize)]
struct BalanceData {
    balance: f32,
}

fn main() {
    let mut amount: f32 = read_balance();
    let mut notes_data: NotesData = read_notes();

    loop {
        println!("Enter your card number:");
        let card_number: u32 = input();

        if card_number == ACCOUNT_NUMBER {
            println!("Enter pin:");
            let card_pin:u32 = input();
            if card_pin == PIN {
                println!("Enter option: 1.credit, 2.debit, 3.check_balance");
                let option: u32 = input();

                match option {
                    1 => {
                        println!("Enter the number of hundred notes to be credited:");
                        let hundred_notes: u32 = input();
                        println!("Enter the number of two thousand notes to be credited:");
                        let two_thousand_notes: u32 = input();

                        let credit_amount = calculate_credit_amount(hundred_notes, two_thousand_notes);
                        amount += credit_amount;

                        notes_data.hundred_notes += hundred_notes;
                        notes_data.two_thousand_notes += two_thousand_notes;

                        println!("Now your current balance is {}", amount);
                        save_balance(amount);
                        save_notes(notes_data);
                        break;
                    }
                    2 => {
                        println!("Enter the amount to be debited:");
                        let debit_amount: f32 = input() as f32;

                        if debit_amount > amount {
                            println!("Insufficient balance!");
                        } else {
                            let (hundred_notes, two_thousand_notes) = calculate_debit_notes(debit_amount, &notes_data);
                            let debited_amount = calculate_debit_amount(hundred_notes, two_thousand_notes);
                            amount -= debited_amount;

                            if hundred_notes <= notes_data.hundred_notes && two_thousand_notes <= notes_data.two_thousand_notes {
                                notes_data.hundred_notes -= hundred_notes;
                                notes_data.two_thousand_notes -= two_thousand_notes;
                                println!("Debited amount: {}", debited_amount);
                                println!("Debited {} hundred notes and {} two thousand notes.", hundred_notes, two_thousand_notes);
                                println!("Now your current balance is {}", amount);
                                save_balance(amount);
                                save_notes(notes_data);
                            } else {
                                println!("Insufficient notes to debit!");
                            }
                        }
                        break;
                    }
                    3 => {
                        println!("Your current balance is {}", amount);
                        break;
                    }
                    _ => println!("Enter a valid option: 1, 2, or 3."),
                }
            } else {
                println!("Pin doesn't match. Try again!");
                break;
            }
        } else {
            println!("Invalid card number!");
            break;
        }
    }
}

fn input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let input: u32 = input.trim().parse().expect("Invalid input");
    input
}

fn read_balance() -> f32 {
    let mut file = File::open("data.json").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read");

    let balance_data: BalanceData = serde_json::from_str(&contents).expect("Failed to read balance");
    balance_data.balance
}

fn save_balance(amount: f32) {
    let balance_data = BalanceData { balance: amount };
    let serialized = serde_json::to_string(&balance_data).expect("Serialization failed");
    let mut file = File::create("data.json").expect("File creation failed");
    file.write_all(serialized.as_bytes()).expect("Write to file failed");
}

fn calculate_credit_amount(hundred_notes: u32, two_thousand_notes: u32) -> f32 {
    let hundred_amount = hundred_notes as f32 * HUNDRED_NOTE_VALUE;
    let two_thousand_amount = two_thousand_notes as f32 * TWO_THOUSAND_NOTE_VALUE;
    hundred_amount + two_thousand_amount
}

fn calculate_debit_amount(hundred_notes: u32, two_thousand_notes: u32) -> f32 {
    let hundred_amount = hundred_notes as f32 * HUNDRED_NOTE_VALUE;
    let two_thousand_amount = two_thousand_notes as f32 * TWO_THOUSAND_NOTE_VALUE;
    hundred_amount + two_thousand_amount
}

fn calculate_debit_notes(amount: f32, notes_data: &NotesData) -> (u32, u32) {
    let mut remaining_amount = amount;
    let mut hundred_notes = 0;
    let mut two_thousand_notes = 0;

    if amount >= TWO_THOUSAND_NOTE_VALUE && notes_data.two_thousand_notes > 0 {
        let available_notes = (amount / TWO_THOUSAND_NOTE_VALUE).min(notes_data.two_thousand_notes as f32);
        let value = available_notes * TWO_THOUSAND_NOTE_VALUE;
        two_thousand_notes = available_notes as u32;
        remaining_amount -= value;
    }

    if amount >= HUNDRED_NOTE_VALUE && notes_data.hundred_notes > 0 {
        let available_notes = (remaining_amount / HUNDRED_NOTE_VALUE).min(notes_data.hundred_notes as f32);
        let value = available_notes * HUNDRED_NOTE_VALUE;
        hundred_notes = available_notes as u32;
        remaining_amount -= value;
    }

    if remaining_amount > 0.0 {
        println!("Cannot debit the amount due to insufficient notes!");
        (0, 0)
    } else {
        (hundred_notes, two_thousand_notes)
    }
}

fn read_notes() -> NotesData {
    let mut file = File::open("notes.json").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read");

    let notes_data: NotesData = serde_json::from_str(&contents).expect("Failed to read notes");
    notes_data
}

fn save_notes(notes_data: NotesData) {
    let serialized = serde_json::to_string(&notes_data).expect("Serialization failed");
    let mut file = File::create("notes.json").expect("File creation failed");
    file.write_all(serialized.as_bytes()).expect("Write to file failed");
}
