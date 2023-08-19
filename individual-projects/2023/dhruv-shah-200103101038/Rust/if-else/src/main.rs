// if condition {
//     statement to execute if condition proves to true
// }
fn main() {
    // example 1
    let num = 50;
    if num < 100 {
        println!("the number is less than 100");
    }

    // example 2
    let marks = 70;
    if marks >= 65 && marks <= 80 {
        println!("it is good");
    }

    // example3
    let flag_1 = true;
    let flag_2 = false;
    if flag_1 == true || flag_2 == true {
        println!("one of the condition is true");
    }
    if flag_1 != false {
        println!("this will execute when flag is true or in otherhand when it is not false")
    }

    // example 4
    let flag_1 = true;
    let flag_2 = false;
    let num = 50;
    if (flag_1 == true) && (flag_2 == false || num < 40) {
        println!("this part will execute base on condition above");
    }

    // 1.if else condition

    // if condition {
    //     statement to execute if condition is true
    // }
    // else {
    //    statement to execute if condition is false
    // }

    // example 1

    let marks = 70;
    if marks > 100 {
        println!("he is pass");
    } else {
        println!("he is failed");
    }

    //2. if else if ladder

    // if condition {
    //     statement to execute if condition is true
    // }
    // else if condition_2 {
    //    statement to execute if condition_2 is true
    // }
    // else {
    //     statement to execute if condition is false
    // }

    // example 1
    let marks = 95;
    let mut _grade = 'D';
    if marks >= 90 {
        _grade = 'A';
    } else if marks >= 80 {
        _grade = 'B'
    } else if marks >= 65 {
        _grade = 'C'
    } else {
        _grade = 'F'
    }
    println!("the grade is {}", _grade);

    
    // example
    println!("Welcome to Guessing number game");

    println!("Would you like to start the game? (y/n)");
    let mut is_start = String::new();

    std::io::stdin()
        .read_line(&mut is_start)
        .expect("Failed to read line.");

    is_start = is_start.trim().to_string();

    if is_start == "y" {
        println!("Please input your guess");
    } else if is_start == "n" {
        println!("Successfully quit the game");
    } else {
        println!("Invalid command");
    };
}
