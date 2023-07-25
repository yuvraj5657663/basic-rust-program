use std::io;

fn main() {
    /* Write a program to find the sum of natural numbers below a given number N, where N is provided by the user. The sum should only include numbers that are multiples of either 3 or 5.

    For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15. Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.

    The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

    Write a program that takes the user input N, performs the necessary calculations, and outputs the sum. */

    println!("Please Enter the number: ");
    let mut _user_input = String::new();

    io::stdin()
        .read_line(&mut _user_input)
        .expect("Failed to read line!");

    let _user_input: i32 = _user_input.trim().parse().expect("Please type a number!");

    let mut sum = 0;
    for i in 1.._user_input {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
            println!("{i}");
        }
    }
    println!("Sum = {sum}");
}
