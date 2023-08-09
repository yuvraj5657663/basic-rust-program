// fn main() {
//     // Immutable variables (परिवर्तनशील)
//     let age: i32 = 30; // Integer type
//     let pi: f64 = 3.14; // Floating-point type
//     let is_student: bool = true; // Boolean type
//     let first_initial: char = 'J'; // Character type

//     // Mutable variable (परिवर्तनशील)
//     let mut count: i32 = 0; // Mutable integer variable
//     count = count + 1; // Mutable variable की value को change किया गया

//     // Variables with inferred types (स्वयं ज्ञात प्रकार वाले वेरिएबल)
//     let city = "New York"; // Rust automatically infers that city is of type &str (string slice)
//     let temperature = 25.5; // Rust automatically infers that temperature is of type f64 (floating-point)

//     println!("Age: {}", age);
//     println!("Pi: {}", pi);
//     println!("Is Student: {}", is_student);
//     println!("First Initial: {}", first_initial);
//     println!("Count: {}", count);
//     println!("City: {}", city);
//     println!("Temperature: {}", temperature);
// }

fn main(){
    let name = "yuvraj";
    let age = 19;
    let pi = 3.1415;
    let yuvraj = true;
    let first_latter_name = 'Y';

    let mut count = 0;
    count = count + 1;
    
    let city = "munger";
    let temp = 32.5;

    println!("my name is: {}", name);
    println!("my age is: {} ",age);
    println!("value of pi: {} ", pi);
    println!("name is : {} ",yuvraj);
    println!(" first_latter of name: {} ",first_latter_name);
    println!(" count : {} ",count);
    println!(" my city is: {} ", city);
    println!("city of temp: {} ",temp);
}
