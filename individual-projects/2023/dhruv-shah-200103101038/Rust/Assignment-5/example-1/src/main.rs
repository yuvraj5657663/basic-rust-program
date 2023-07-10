// Write a program for finding the difference of the square of the sum and the sum of square of the first N number
// (where N is a user defined input that you program will take). for instance,
//  if the user enters the number of let say 5,
// Then you should first compute the squae of sum = (1+2+3+4+5)^2  = 225
// and next you will compute the sum of square as  = (1^2  + 2^2  + 3^2  + 4^2  + 5^2)  = (1 + 4+ 9 + 16 +25 ) = 55
// and finally you will compute the difference = 225 - 55 = 170.

fn main() {
    println!("enter a number");
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        square_of_sum += i;
        sum_of_squares += i.pow(2);
    }
    println!("sum of first {} numbers is {}", n, square_of_sum);
    println!(
        "sum of squares of first {} numbers is {}",
        n, sum_of_squares
    );

    let difference = square_of_sum.pow(2) - sum_of_squares;
    println!("The difference of the both = {} is {}", n, difference);
}
