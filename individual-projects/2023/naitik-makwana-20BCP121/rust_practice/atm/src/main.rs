// Project : ATM Machine

/*
Create a simple banking system. It allows users to check their balance, withdraw money, and deposit money. The program stores user account information in a JSON file.
*/

#[derive(Debug)]

struct Customer {
    customer_id : u64,
    card_num : u64,
    pin : u64,
    balance : u64 
}

fn take_input () -> u64 {

    let mut op = String::new();

    std::io::stdin().read_line(&mut op).expect("please Enter numeric value");
    let op = op.trim().parse::<u64>().expect("Unable to read input");
    op
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

fn main() {

    let mut customer_1 = Customer {
        customer_id : 12345,
        card_num : 100120023003,
        pin: 1234,
        balance: 235000
    };

    let mut retry = 0;
    let mut  wrong_pass_counter = 5;
    let mut operation = 0;
    let mut amount = 0;

    
    println!("##### SBI ATM #####");

    loop {
        println!("Enter your card number: ", );
        let card_num = take_input();
    
        if customer_1.card_num==card_num {
        
            println!("Enter your pin: ", );
            let pin = take_input();
            
            if customer_1.pin==pin {
                println!("What do you want to do? \n 1. Credit \n 2. Debit");
                operation=take_input();
                match operation {
                    1 => {
                        println!("Enter amount : ");
                        amount = take_input();
                        customer_1.credit(amount);
                        println!("New balance : {}", customer_1.balance);
                        println!("Thank you !!");
                        break;
                    },
                    2 => {
                        println!("Enter amount : ");
                        amount = take_input();
                        if amount <= customer_1.balance {
                            customer_1.debit(amount);
                            println!("Transaction successful !! \nNew balance : {}", customer_1.balance);
                            println!("Thank you !!");
                            break;
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
            println!("Wrong card number !!");
            println!("Select option below : \n 1. retry 2. cancel transaction");
            retry = take_input();
            if retry == 1 {
            }else {
                break;
            }
        }
    }

}