/*[1]. Write a program for finding the difference of the square of the sum and the sum of square of the first N number (where N is a user defined input that you program will take). for instance, if the user enters the number of let say 5,
Then you should first compute the squae of sum = (1+2+3+4+5)^2  = 225

and next you will compute the sum of square as  = (1^2  + 2^2  + 3^2  + 4^2  + 5^2)  = (1 + 4+ 9 + 16 +25 ) = 55

// and finally you will compute the difference = 225 - 55 = 170. */

// fn main() {
//     let mut sum_of_squares = 0;
//     let mut square_of_sum = 0;
//     let mut sum = 0;
//     let mut n = String::new();

//     println!("Enter the value of number :");
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("Failed to read line");
//     let n: u32 = n.trim().parse().expect("Please type a number!");

//     for i in 1..=n {
//         sum_of_squares = sum_of_squares + i.pow(2);
//         sum = sum + i;
//         square_of_sum = sum.pow(2);
//     }
//     println!(
//         "Sum of Square is : {}\nSquare of Sum is : {}",
//         sum_of_squares, square_of_sum
//     );
//     println!("The difference is {}", square_of_sum - sum_of_squares);
// }

/* [2] Find the sum of natural numbers below number N (where N is provide by user) that are multiples of either 3 or 5.  For example, if the user enters a number N = 20 then
multiples of 3 = 3,6,9,12,15,18
multiples of 5 = 5, 10, 15

Sum = 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18   (Please note that value of 15 will be counted once since it is multiple of both 3 and 5) */
// fn main() {
//     let mut n = String::new();
//     println!("Enter the Number : ");
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("Failed to read input");
//     let n: u32 = n.trim().parse().expect("Please type a number!");
//     let mut sum = 0;
//     for i in 1..n {
//         if i % 3 == 0 || i % 5 == 0 {
//             sum = sum + i;
//         }
//     }
//     println!("The sum of mulriple of 3 and 5 are : {}", sum);
// }

/* [3]. This question is about writing some code to analyze the production of an assembly line in a car factory. The assembly line have different speeds which can range from 0 (off) to 10 (maximum).  At its lowest speed that is the 1, a total of 221 cars are produced each hour. The production increases linearly with the speed. This means that when the speed is set to 4, it should produce 4 * 221 = 884 cars per hour. However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded. The following table shows how speed influences the success rate:
1 to 4: 100% success rate.
5 to 8: 90% success rate.
9 and 10: 77% success rate.
You are requied to write two functions for the following two scenarios.
1. Write a function called total_production() which will calculate the assembly line's total production in some specified time given in hours, taking into account its success rate. The input to the function will be the number of hours and speed while the output will be the number of cars successfully produced without the faults.
2. Write another function called Cars_produced_per_minutes(). The input to the function will be the hours and speed while the output wil be the number of cars successfully produced per minutes.
*/
/* ANS */
// const CARS_PER_HOURS: u32 = 221;
// fn main() {
//     let mut hours = String::new();
//     let mut speed = String::new();
//     println!("Enter the Hours : ");
//     std::io::stdin()
//         .read_line(&mut hours)
//         .expect("Failed to read input");
//     let hours: u32 = hours.trim().parse().expect("Please type a number!");

//     println!("Enter the Speed : ");
//     std::io::stdin()
//         .read_line(&mut speed)
//         .expect("Failed to read input");
//     let speed: u32 = speed.trim().parse().expect("Please type a number!");

//     let total_production = total_production(hours, speed);
//     println!("Total Production is : {}", total_production);

//     let cars_produced_per_minutes = cars_produced_per_minutes(hours, speed);
//     println!(
//         "Cars Produced Per Minutes is : {}",
//         cars_produced_per_minutes
//     );
// }

// fn total_production(hours: u32, speed: u32) -> u32 {
//     match speed {
//         1..=4 => CARS_PER_HOURS * hours * speed,
//         5..=8 => CARS_PER_HOURS * hours * speed * 90 / 100,
//         9..=10 => CARS_PER_HOURS * hours * speed * 77 / 100,
//         _ => 0,
//     }
// }

// fn cars_produced_per_minutes(hours: u32, speed: u32) -> u32 {
//     total_production(hours, speed) / 60
// }

/*[4] Palindrome is a word, verse, or sentence (such as "Able was I ere I saw Elba") or a number (such as 1881) that reads the same backward or forward. Write a function called palindrome which will check if a given string is a palindrome or not. The input to the function will be String and the output will be a bool value.  */
/* ANS */

// fn main() {
//     let mut input = String::new();
//     println!("Enter the String : ");
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read input");
//     let input = input.trim();
//     let palindrome = palindrome(input);
//     println!("The String is Palindrome : {}", palindrome);
// }

// fn palindrome(input: &str) -> bool {
//     let input = input.to_lowercase();
//     let reverse = input.chars().rev().collect::<String>();
//     if input == reverse {
//         true
//     } else {
//         false
//     }
// }

/*[5] A Pythagorean triple consists of three positive integers a, b, and c, such that a*a + b*b = c*c. Such a triple is commonly written as (a, b, c), and a well-known example is (3, 4, 5). Write a program that will compute the Pythagorean triplet such that a < b < c and a+b+c = 1000. */
/* ANS */

// fn main() {
//     let mut a = 0;
//     let mut b = 0;
//     let mut c = 0;
//     let mut sum = 0;
//     for i in 1..1000 {
//         for j in 1..1000 {
//             for k in 1..1000 {
//                 if i < j && j < k {
//                     if i * i + j * j == k * k {
//                         if i + j + k == 1000 {
//                             a = i;
//                             b = j;
//                             c = k;
//                             sum = a + b + c;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     println!("The Pythagorean triplet such that a < b < c and a+b+c = 1000 is : {}, {}, {} and sum is : {}", a, b, c, sum);
// }

/*[6] Write a function that implements the logic, “You can see the movie if you are 17 or older, or you’re 13 or older and have a parent’s permission.”
Use the following skeleton of the function. Remove the return false statement once you write the code inside the function
fn can_see_movie(age: i32, permission: bool) -> bool {
return false
} */
/* ANS */
fn main() {
    let mut age = String::new();
    println!("Enter the Age: ");
    std::io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");
    let age: i32 = age.trim().parse().expect("Please enter a number!");

    let can_see = can_see_movie(age);

    if can_see {
        println!("You can see the movie.");
    } else {
        println!("You can't see the movie.");
    }
}

fn can_see_movie(age: i32) -> bool {
    if age >= 17 {
        return true;
    } else if age >= 13 {
        let mut permission = String::new();
        println!("Do you have permission? (true or false): ");
        std::io::stdin()
            .read_line(&mut permission)
            .expect("Failed to read input");
        let permission: bool = permission
            .trim()
            .parse()
            .expect("Please enter either true or false!");

        if permission {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}
