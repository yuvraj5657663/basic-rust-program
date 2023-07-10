fn main() {

    println!("Enter the width of  rectengle : ");
//geting input for width

    let mut p = String::new();

    std::io::stdin()
    .read_line(&mut p)
    .expect(" failed to  input");

    let width:u32 = p.trim().parse().expect("try with vellid input");

// geting input for length
  
    println!("Enter the lengthof  rectengle :");

    let mut p = String::new();

    std::io::stdin()
    .read_line(&mut p)
    .expect(" failed to  input");

    let length:u32 = p.trim().parse().expect("try with vellid input");
    let area_of_rectengle = area(length,width);
    println!("Area of the rectengle is {}.",area_of_rectengle);
    

}
fn area( length:u32,width:u32) -> u32{
    length * width

}


/* Question3.4
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

 } */