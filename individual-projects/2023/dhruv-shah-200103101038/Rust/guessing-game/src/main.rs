//Programming a Guessing Game
//The program will generate a random integer between 1 and 100

// Processing a Guess
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generating a Random Number
    let _secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {_secret_number}");

    // Allowing Multiple Guesses with Looping
    loop {
        println!("Please input your guess.");

        // Storing Values with Variables
        let mut guess = String::new();

        // Receiving User Input
        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");

        // let guess: u32 = guess.trim().parse().expect("please type number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        // Comparing the Guess to the Secret Number
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
