// Section-2 // Assignment-3

/* Problem - 4
Complete the following code given below by filling in the code corresponding to the  comments

 fn main(){
    // Step 1: Write a print statement for asking user to input the width of a rectangle
    // Your code here

    // Step 2: Write code for taking the input from the user of type u32 and store it in the variable of width
    // your code here

    // Step 3: Write a print statement for asking the user to input the length of a rectangle
    // Your code here

    // Step 4: Write code for taking the input from the user of type u32 and store it in the variable of length
    // your code here  

    let resultant_area = {
        // Step 5: call a function area() inside here with inputs of width and length which will return the area
    };

    // Step 6: write code to print the resultant_area variable to the terminal

}

fn area(length:u32, width:u32) -> u32 {

    // Step 7: write the definition of the area here which is length * width and return the result
}
*/
use std::io;
fn main () {
    
    println!("Enter the width of rectangle");
    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("failed");
    let width:u32 = width.trim().parse().unwrap();


    println!("Enter the length of rectangle");
    let mut length = String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("failed");
    let length:u32 = length.trim().parse().unwrap();

    let resultant_area = {
        area(length,width)
    };

    println!("area of rectangle = {}", resultant_area);

}

fn area(length:u32, width:u32) -> u32 {
    length*width
}
