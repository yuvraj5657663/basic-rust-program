// 1. nested if
fn main() {
    /*
    if outer_condition {
        // statement is execute if outer is true
        if inner_condition {
            // statement is execute if both the outer and inner is true
        } else {
            // some statement to execute
        }
    else {
            // some statement to execute
        }
    }
    */

    // example 1
    println!("enter a number");
    let mut num = String::new();
    std::io::stdin()
        .read_line(&mut num)
        .expect("failed to read line");
    let num: i32 = num.trim().parse().expect("invalid input");

    if num != 0 {
        if num % 2 == 0 {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    } else {
        println!("{} is zero", num);
    }

    // 2. if let
    // let variable = if condition {
    //     // statement to execute if condition is true
    //     value_1
    // } else {
    //     // statement to execute if condition is false
    //     value_2
    // };

    // example 1
    let marks = 95;
    let _grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 65 {
        'C'
    } else {
        'F'
    };

    println!("the grade is {}", _grade);
}
