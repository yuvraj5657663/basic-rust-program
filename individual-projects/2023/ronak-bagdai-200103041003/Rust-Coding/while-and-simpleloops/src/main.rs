fn main() {
    // loop {
    //     println!("Hello, world!");
    // }

    // let my_number = 5;
    // let mut guess = false;

    // println!("Guess the number!");

    // while guess != true {
    //     let mut number = String::new();
    //     std::io::stdin()
    //         .read_line(&mut number)
    //         .expect("Failed to read line");

    //     let number:u8 = number.trim().parse().expect("invalid input");

    //     if my_number == number {
    //         println!("You guessed it!");
    //         guess = true;
    //     }
    //     else {
    //         println!("Try again!");
    //     }
    // }

    println!("Plz enter the number and i will tell you the next number after your number that is divisible by both 2 and 5");

    let mut _number = String::new();

    std::io::stdin()
        .read_line(&mut _number)
        .expect("Failed to read line");

    let mut _number: u8 = _number.trim().parse().expect("invalid input");
    // let mut divisible_by_2_5 = false;

    _number = _number + 1;
    while (_number % 2 == 0 && _number % 5 == 0) != true {
        _number = _number + 1;
    }
    println!(
        "The next number after your number divisible by both 2 and 5 is {}",
        _number
    );
}
