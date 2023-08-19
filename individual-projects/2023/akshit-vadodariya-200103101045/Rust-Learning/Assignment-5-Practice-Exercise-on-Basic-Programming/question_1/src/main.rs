use std::io;
fn main() {
    /* Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers. N will be a user-defined input that your program will take.

    For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.

    Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.

    Finally, calculate the difference as 225 - 55 = 170. */

    println!("Please enter the number: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: i32 = user_input.trim().parse().expect("Please type a number!");

    let mut _sum: i32 = 0;
    let mut _square: i32 = 0;
    for _i in 1..=user_input {
        _sum += _i;
        _square += _i.pow(2);
    }
    println!("Square of the sum is: {}", _sum.pow(2));
    println!("sum of the square is: {}", _square);
    let _difference = _sum.pow(2) - _square;
    println!("Difference as {} - {} = {}", _sum.pow(2), _square, _difference);
}
