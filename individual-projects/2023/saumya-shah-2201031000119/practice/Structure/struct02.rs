struct Account {
    name: String,
    account_number: String,
    balance: f32,
    account_type: String,
}

fn main() {
    let info = Account {
        name: String::from("FirstName-LastName"),
        account_number: String::from("987456321"),
        balance: 100000.00,
        account_type: String::from("Savings"),
    };
    println!("Name : {}", info.name);
    println!("Account Number : {}",info.account_number);
    println!("Balance in your account : {}", info.balance);
    println!("Your account type : {}", info.account_type);
}