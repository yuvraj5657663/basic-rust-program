fn holiday() {
    return true;
}

fn main() {
    let today = date(); // current date
    if today == holiday() {
        println!("Enjoy the holiday!");
    }
    else {
        println!("Go to Office");
    }
}