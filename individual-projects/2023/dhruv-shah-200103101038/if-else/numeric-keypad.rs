fn main(){
    let a = keypad;
    if (a == "a-z,A-Z") {
       println!("Keypad is not numeric");
    }
    else if (a == "0-9") {
        println!("Keypad is numeric");
    }
    else {
        println!("invalid value");
    }
}

