// Project : ATM Machine

/*
Create a simple banking system. It allows users to check their balance, withdraw money, and deposit money. The program stores user account information in a JSON file.
*/

/*Add support of "number of notes in atm" code.
e.g ATM should be aware with how many notes are available as per the balance.
2 notes of 2000
5 notes of 500
and total balance is 6500.
And credit and debit should work accordingly. */

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read,Write};

#[derive(Debug,Serialize,Deserialize)]
struct Customer {
    customer_id : u64,
    card_num : u64,
    pin : u64,
    balance : u64
}

impl Customer {
    fn credit (&mut self, money : u64) -> u64 {
        self.balance += money;
        self.balance
    }
}

impl Customer {
    fn debit (&mut self, money: u64) -> u64 {
        self.balance -= money;
        self.balance
    }
}


#[derive(Debug,Serialize,Deserialize)]
struct Atm {
    two_k : u64,
    five_h : u64,
    avail_bal_atm: u64 
}

impl Atm {
    fn credit(&mut self, two_k : u64, five_h : u64) {
        self.two_k += two_k;
        self.five_h += five_h;
        self.avail_bal_atm += two_k*2000 + five_h*500;
    }
}

impl Atm {
    fn debit(&mut self, two_k : u64, five_h : u64) {
        self.two_k -= two_k;
        self.five_h -= five_h;
        self.avail_bal_atm -= two_k*2000 + five_h*500;
    }
}


fn take_input () -> u64 {

    let mut op = String::new();
    std::io::stdin().read_line(&mut op).expect("please Enter numeric value");
    let op = op.trim().parse::<u64>().expect("Unable to read input");
    return op
} 

fn read_json_typed_cust(raw_json: &str) -> Vec<Customer> {
    
    //reading json type data and converting it to  vector of  Custromer type instances
    let customers_list: Vec<Customer> = serde_json::from_str(raw_json).expect("Error while deserialization"); 
    customers_list
}

fn read_json_typed_atm(raw_json: &str) -> Atm {   
    //reading json type data and converting it to ATM type instance
    let atm_1 : Atm = serde_json::from_str(raw_json).unwrap();
    return atm_1
}

fn main() {
    //opening custoner json File
    let mut cust_json_file_data = File::open("customer_data.json").unwrap();

    let mut cust_content = String::new();
    //reading data from customer_data json
    cust_json_file_data.read_to_string(&mut cust_content).expect("Error");
    let cust_json_objects = cust_content.as_str(); //String to &str
    //converting json to vector of customer type structs
    let mut customers_list: Vec<Customer> = read_json_typed_cust(cust_json_objects); 
    
    //opening atm json File
    let mut atm_json_file_data = File::open("atm_data.json").unwrap();
    let mut atm_content = String::new();
    //reading data from atm_data json
    atm_json_file_data.read_to_string(&mut atm_content).expect("Error");
    let atm_json_object = atm_content.as_str(); //String to &str  
    let mut atm_1: Atm = read_json_typed_atm(atm_json_object); //creating Customer instance

    let mut retry = 0;
    let mut wrong_pass_counter = 5;
    let mut operation = 0;
    let mut amount = 0;
    let mut two_k_notes = 0;
    let mut five_h_notes = 0;
    
    println!("##### SBI ATM #####");

    loop {
        println!("Enter your card number: ", );
        let card_num = take_input();
        
        //iterating through loop to check with if let condtition to create customer instance if card number matches
        if let Some(target_customer) = customers_list.iter_mut().find(|cust| cust.card_num == card_num) {
    
            println!("Enter your pin: ", );
            let pin = take_input();
            
            //cheching pin
            if target_customer.pin==pin {
                println!("What do you want to do? \n 1. Credit \n 2. Debit");
                operation=take_input();
                match operation {
                    1 => {
                        println!("Enter amount in mupltiple of 500 : ");
                        amount = take_input();
                        // to check if entered amount is in multiple of 500
                        if amount%500 == 0 {
                            //taking input for 2000 rupees notes
                            println!("Enter total no of 2,000 Rupees note : ");
                            two_k_notes = take_input();
                            if two_k_notes*2000 > amount {
                                println!("greater value than inserted amount");
                                break;
                            }
                            //taking input for 500 rupees notes
                            println!("Enter total no of 500 Rupees note : ");
                            five_h_notes = take_input();
                            if five_h_notes*500 > (amount - (two_k_notes*2000)){
                                println!("greater value than inserted amount");
                                break;
                            }

                            if two_k_notes*2000 + five_h_notes*500 != amount {
                                println!("You have inserted lesser amount than you inserted");
                                break;
                            }

                            target_customer.credit(amount);
                            atm_1.credit(two_k_notes,five_h_notes);

                            println!("New balance : {}", target_customer.balance);
                            println!("Thank you !!");
                            break;
                        }else {
                            println!("Amount should be in multiple of 500. Try again!!");
                            break;
                        }

                    },
                    2 => {
                        println!("Enter amount in muplitple of 500 : ");
                        amount = take_input();

                        //to check if entered amount is lesser than equal to available balance in customer account
                        if amount <= target_customer.balance {
                            // to check if entered amount is in multiple of 500
                            if amount%500 == 0 {
                                //to check if entered amount is leeser than available balance in atm machine
                                if amount <= atm_1.avail_bal_atm {
                                    
                                    let mut amount_to_be_dispense:u64 = amount;

                                    //as per this code it will try to give you maximum amount in 2k notes and rest in 500
                                    while amount_to_be_dispense >= 2000 { 
                                        two_k_notes += 1;
                                        amount_to_be_dispense -= 2000;
                                        if two_k_notes == atm_1.two_k {
                                            break;
                                        }
                                    }

                                    while amount_to_be_dispense >= 500 {
                                            five_h_notes += 1;
                                            amount_to_be_dispense -= 500;
                                    } 

                                    println!("Total amount debited : {}", amount);
                                    println!("total {} notes dispensed which contains {} : 2000 notes and {} : 500 notes", two_k_notes+five_h_notes, two_k_notes, five_h_notes);
        
                                    target_customer.debit(amount);
                                    atm_1.debit(two_k_notes,five_h_notes);

                                    println!("Transaction successful !! \nNew balance : {}", target_customer.balance);
                                    println!("Thank you !!");
                                    break;
                                }
                                else {
                                    println!("Can't debit entered amount,total amount available in this ATM is : {}", atm_1.avail_bal_atm);
                                    break;
                                }
                            }else {
                                println!("Amount should be in multiple of 500. Try again!!");
                                break;
                            }

                        }else {
                            println!("Amount is greater than your balance. Try again!!");
                        }
                    },
                    _ => {
                        println!("Wrong selection try again." ); 
                        break;
                    }

                }
                
                }else {
                    wrong_pass_counter-=1;
                    println!("Wrong pin!! {} attempts left", wrong_pass_counter );
                    if wrong_pass_counter == 0 {
                        println!("Start again.");
                        break;
                    }
                }
        }else {
            println!("Entered carn number is not valid");
            println!("Select option below : \n 1. retry 2. cancel transaction");
            retry = take_input();
            if retry == 1 {
            }else {
                break;
            }
        }
        
    }

    let to_cust_json_objects = serde_json::to_string(&customers_list).expect("Failed to serialize data"); //converting Customer type to json object
    std::fs::write("customer_data.json",to_cust_json_objects).expect("failed to write into json file"); // writing into customer json file

    let to_atm_json_object = serde_json::to_string(&atm_1).unwrap(); //converting Atm type to json object
    let mut atm_json_file = File::create("atm_data.json").unwrap();
    atm_json_file.write(to_atm_json_object.as_bytes()).expect("Error"); //storing data into atm json
}