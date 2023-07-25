/* Write a function that implements the logic, 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'
Use the following skeleton of the function. Remove the 'return false' statement once you have written the code inside the function.
fn can_see_movie(age: i32, permission: bool) -> bool {

    // Write your code here

    // Remove 'return false' once you have written the code

    return false;

}

Please fill in the code inside the function to implement the logic described above. Once you have completed the implementation, remove the 'return false' statement. */

use std::io;

fn can_see_movie(age: i32, _permission: i32) -> bool {
    if age >= 13 && _permission == 1 {
        println!("Have parent's permission");
    } else {
        println!("You can't see the movie");
    }
    return false;
}

fn main() {
    let mut age = String::new();
    println!("Enter your age: ");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age: i32 = age.trim().parse().expect("Please type number!");

    if age >= 17 {
        println!("You can see the movie");
    } else if age >= 13 {
        let mut permissions = String::new();
        println!("You have parent's permission? : ");
        println!("Yes, press: 1");
        println!("No, press: 0");
        io::stdin()
            .read_line(&mut permissions)
            .expect("Failed to read line");
        let permissions: i32 = permissions.trim().parse().expect("Please type number");
        can_see_movie(age, permissions);
    } else {
        println!("You can't see the movie");
    }
}
