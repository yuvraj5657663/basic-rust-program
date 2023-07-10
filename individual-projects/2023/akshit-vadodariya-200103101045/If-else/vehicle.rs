fn main() {
    println!("1.Petrol");
    println!("2.Diesel");

    let choose = user_input;
    if choose == 1 {
        println!("Your vehicle is Petrol");
    }
    else if choose == 2 {
        println!("Your vehicle is Diesel");
    }
    else {
        println!("Wrong input!");
    }
}