use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("### Welcome to the guessing game ###");
    
    //generating num between 1 to 100 using random function.
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guessed number : ");
    
        //taking input from user
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error! not able to read input");
    
        //converting String to integer. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess is : {}", guess);
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("It is smaller. Try with bigger one."),
            Ordering::Greater => println!("It is bigger. Try with smaller one"),
            Ordering::Equal => {
                println!("You guessed it right! you won the game!");
                break;
            }
        }
    }

}
