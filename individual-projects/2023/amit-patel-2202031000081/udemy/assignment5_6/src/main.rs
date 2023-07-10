use std::fs::Permissions;

fn main() {
    println!("Enter your age:");
    let mut age=String::new();

    std::io::stdin().read_line(&mut age)
    .expect("invalid input");

    let age: i32 = age
    .trim().parse()
    .expect("invalid input");

if age >= 17
{
    println!("You can see the movie!!");
}
else 
{
    let mut Permissions = String::new();
    println!("Enter your parent permission in true or false:{}",Permissions );
    std::io::stdin()
    .read_line(&mut Permissions)
    .expect("Invallid input");

let mut Permissions:bool=Permissions
.trim()
.parse()
. expect("invalid input");   

let mut permission_granted:bool= true;
if permission_granted == can_see_movie(age, Permissions)
{
    println!("/n You can see  movie  child!!/n");

}
else
{
    println!("You are too little to see movie!! ");
}
 

   
}
 
}
fn can_see_movie(age: i32, permission: bool) -> bool {

   if  (age >= 17) || (age >= 13  && permission == true)
    {
         true
    }

   else 
    {
         false
    }
}
        

/*
Write a function that implements the logic, “You can see the movie if you are 17 or older, or you’re 13 or older and have a parent’s permission.”

Use the following skeleton of the function. Remove the return false statement once you write the code inside the function 

fn can_see_movie(age: i32, permission: bool) -> bool {

return false

} */
