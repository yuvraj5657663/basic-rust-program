fn main() {
    // Step 1: Write a print statement for asking user to input the width of a rectangle
    println!("Please enter the width of the rectangle: ");

    // Step 2: Write code for taking the input from the user of type u32 and store it in the variable of width
    let mut width = String::new();
    std::io::stdin()
        .read_line(&mut width)
        .expect("failed input");
    let width: u32 = width.trim().parse().expect("failed input");

    // Step 3: Write a print statement for asking the user to input the length of a rectangle
    println!("Please enter the length of the rectangle: ");

    // Step 4: Write code for taking the input from the user of type u32 and store it in the variable of length
    let mut length = String::new();
    std::io::stdin()
        .read_line(&mut length)
        .expect("failed input");
    let length: u32 = length.trim().parse().expect("failed input");
    
    // Step 5: call a function area() inside here with inputs of width and length which will return the area
    let resultant_area = area(length, width);

    // Step 6: write code to print the resultant_area variable to the terminal
    println!(
        "The area of the rectangle is {} square units",
        resultant_area
    );
}

fn area(length: u32, width: u32) -> u32 {
    // Step 7: write the definition of the area here which is length * width and return the result
    length * width
}
