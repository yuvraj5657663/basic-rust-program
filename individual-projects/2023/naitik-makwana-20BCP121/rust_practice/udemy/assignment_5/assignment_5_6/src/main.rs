// Section-4 // Assignment-5

/* Problem - 6
Write a function that implements the logic, “You can see the movie if you are 17 or older, or you’re 13 or older and have a parent’s permission.”

Use the following skeleton of the function. Remove the return false statement once you write the code inside the function 

fn can_see_movie(age: i32, permission: bool) -> bool {

return false

}
*/

fn can_see_movie(age: u32, permission: bool) -> bool {

    if age>=13 && permission==true {
        true
    }else{
        false
    }    
}

fn main() {

    println!("Enter your age");
    
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).expect("fail");
    let age = age.trim().parse::<u32>().unwrap();

    if age >= 17 {
        println!("You can not see movie");
    } else if age >= 13 {
        println!("Enter your parent's permission :  true or false");
        let mut permission = String::new();
        std::io::stdin().read_line(&mut permission).expect("fail");
        let permission = permission.trim().parse::<bool>().unwrap();

        println!("Your permission is : {}", can_see_movie(age,permission));

    }else {
        println!("You can not see movie");
    }

}