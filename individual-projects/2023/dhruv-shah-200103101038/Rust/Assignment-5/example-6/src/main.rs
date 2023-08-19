// Certainly! Here's the function that implements the logic
// "You can see the movie if you are 17 or older,
// or you’re 13 or older and have a parent's permission"
// Use the following skeleton of the function. Remove the return false statement once you write the code inside the function
// fn can_see_movie(age: i32, permission: bool) -> bool {
// return false
// }

fn main() {
    let mut age = String::new();
    println!("Enter your Age: ");
    std::io::stdin().read_line(&mut age).expect("");
    let age: i32 = age.trim().parse().expect("");

    let can_see = can_see_movie(age);

    if can_see {
        println!("You can see the movie.");
    } else {
        println!("You can't see the movie.");
    }
}

fn can_see_movie(age: i32) -> bool {
    if age >= 17 {
        true;
    } else if age >= 13 {
        let mut permission = String::new();
        println!("Do you have permission? (true or false): ");
        std::io::stdin().read_line(&mut permission).expect("");
        let permission: bool = permission.trim().parse().expect("");

        if permission {
            true;
        }
        else {
            false;
        }
    }
    false
}
