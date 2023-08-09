// what is ccontrol flow in rust
// Control flow is the order in which statements are executed or evaluated.
// Rust provides several control flow operators, including:
// if expression
// if...else expression
// if...else if...else expression
// match expression
// loop expression
// while expression
// for expression

// what is if expression in rust?
// An if expression is used to evaluate a condition and execute a block of code if the condition is true.
// The following example shows an if expression and syntax:
// if condition {
//     // code here
// }

// ***********example of if expression in rust***********

// fn main() {
//     let x = 5;
//     if x == 5 {
//         println!("x is equal to 5");
//     }

// fn main(){
//     let number  = 3;
//     if number != 0 {
//         println!("number was sommething other than zero");
//     }
// }

// what is if...else expression in rust?
// An if...else expression is used to evaluate a condition and execute a block of code if the condition is true, and another block of code if the condition is false.
// The following example shows an if...else expression and syntax:
// if condition {
//     // code here
// } else {
//     // code here
// }

// ***********example of if...else expression in rust***********

// fn main() {
//     let x = 5;
//     if x == 5 {
//         println!("x is equal to 5");
//     } else {
//         println!("x is not equal to 5");
//     }
// }

// ***********example of if...else expression in rust -2 ***********

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is: {}", number);
// }

// what is if...else if...else expression in rust?
// An if...else if...else expression is used to evaluate multiple conditions.
// The following example shows an if...else if...else expression and syntax:
// if condition_1 {
//     // code here
// } else if condition_2 {
//     // code here
// } else {
//     // code here
// }

// ***********example of if...else if...else expression in rust***********

// fn main() {
//     let x = 5;
//     if x == 5 {
//         println!("x is equal to 5");
//     } else if x == 6 {
//         println!("x is equal to 6");
//     } else {
//         println!("x is not equal to 5 or 6");
//     }
// }

// ********else if exaaample-2*****

// fn main (){
//     let number = 6;
//     if number %4==0{
//         println!("number is divisible by 4");
//     }else if number %3==0{
//         println!("number is divisible by 3");
//     }else if number %2==0{
//         println!("number is divisible by 2");
//     }else{
//         println!("number is not divisible by 4,3,2");

//     }
// }

// what is match expression in rust?
// A match expression is used to compare a value against a series of patterns and execute a block of code based on the pattern that matches.
// The following example shows a match expression and syntax:
// match value {
//     pattern_1 => {
//         // code here
//     },
//     pattern_2 => {
//         // code here
//     },
//     pattern_3 => {
//         // code here
//     },
//     _ => {
//         // code here
//     }
// }

// ***********example of match expression in rust***********

// fn main() {
//     let x = 5;
//     match x {
//         1 => println!("x is equal to 1"),
//         2 => println!("x is equal to 2"),
//         3 => println!("x is equal to 3"),
//         4 => println!("x is equal to 4"),
//         5 => println!("x is equal to 5"),
//         _ => println!("x is not equal to 1, 2, 3, 4, or 5"),
//     }
// }

// what is loop expression in rust?
// A loop expression is used to execute a block of code repeatedly until a condition is met.
// The following example shows a loop expression and syntax:
// loop {
//     // code here
// }

// ***********example of loop expression in rust***********

// fn main() {
//     let mut x = 5;
//     loop {
//         x += x - 3;
//         println!("{}", x);
//         if x % 5 == 0 { break; }
//     }
// }

// ***********example of loop expression in rust-2***********
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("the result is {result}");
// }

// loop lebles to disambiguate between multiple loops
// what is multiple loop in rust?
// A multiple loop is used to execute a block of code repeatedly until a condition is met.
// The following example shows a multiple loop and syntax:
// 'outer: loop {
//     'inner: loop {
//         // code here
//         break 'outer;
//     }
// }

// ***********example of multiple loop in rust***********
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("end count = {count}");
// }

// what is while expression in rust?
// A while expression is used to execute a block of code repeatedly as long as a condition is true.
// The following example shows a while expression and syntax:
// while condition {
//     // code here
// }

// ***********example of while expression in rust***********

// fn main() {
//     let mut x = 5;
//     while x != 0 {
//         println!("{}", x);
//         x -= 1;
//     }
// }

// ***********example of while expression in rust-2***********
// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{number}!");
//         number -= 1;
//     }
//     println!("LIFTOFF!!");
// }

// what is for expression in rust?
// A for expression is used to loop over the elements of a collection.
// The following example shows a for expression and syntax:
// for element in collection {
//     // code here
// }

// ***********example of for expression in rust***********

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }

// what is for loop in rust ?
// A for loop is used to loop over the elements of a collection.
// The following example shows a for loop and syntax:
// for number in (1..4).rev() {
//     // code here
// }


// ***********example of for expression in rust-2***********

// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");   
//     }
//     println!("LIFTOFF!!");
// }
