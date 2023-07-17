use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Well_Come to guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Enter your guess number btw 1 to 100.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small number then secret_number!"),
            Ordering::Greater => println!("Too big number then secret_number!"),
            Ordering::Equal => {
                println!("Hurrah!! You get it!");
                break;
            }
        }
    }
}