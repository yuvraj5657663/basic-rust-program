// scalars data types
// 1. integer types
// 2. floating point types
// 3. boolean types
// 4. character types
 
// ********** integer types **********
// each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
// unsigned variant can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 255 inclusive.

 // example integer types problem

//  fn main (){
//  let i8: i8 = 127;
//     let i16: i16 = 32767;
//     let i32: i32 = 2147483647;
//     let i64: i64 = 9223372036854775807;
//     let i128: i128 = 170141183460469231731687303715884105727;
//     let isize: isize = 9223372036854775807;
//     let u8: u8 = 255;
//     let u16: u16 = 65535;
//     let u32: u32 = 4294967295;
//     let u64: u64 = 18446744073709551615;
//     let u128: u128 = 340282366920938463463374607431768211455;
//     let usize: usize = 18446744073709551615;
//     println!("i8: {}", i8);
//     println!("i16: {}", i16);
//     println!("i32: {}", i32);
//     println!("i64: {}", i64);
//     println!("i128: {}", i128);
//     println!("isize: {}", isize);
//     println!("u8: {}", u8);
//     println!("u16: {}", u16);
//     println!("u32: {}", u32);
//     println!("u64: {}", u64);
//     println!("u128: {}", u128);
//     println!("usize: {}", usize);
//  }

 // example floating point types problem

//  fn main (){
//     let f32: f32 = 3.40282347e+38;
//     let f64: f64 = 1.7976931348623157e+308;
//     println!("f32: {}", f32);
//     println!("f64: {}", f64);
//  }

// example floating point types problem-2

//  fn main (){
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
//     println!("x: {}", x);
//     println!("y: {}", y);
//  }

 // example of numeric operations

//  fn main (){
//     // addition
//     let sum = 5 + 10;
//     println!("sum: {}", sum);
//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("difference: {}", difference);
//     // multiplication
//     let product = 4 * 30;
//     println!("product: {}", product);
//     // division
//     let quotient = 56.7 / 32.2;
//     println!("quotient: {}", quotient);
//     // remainder
//     let remainder = 43 % 5;
//     println!("remainder: {}", remainder);
//  }

// example of boolean types
 
//  fn main (){
//     let t = true;
//     println!("t: {}", t);
//     let f: bool = false; // with explicit type annotation
//     println!("f: {}", f);
//  }


// example of character types


//  fn main (){
//     let c = 'z';
//     println!("c: {}", c);
//     let z = 'ℤ';
//     println!("z: {}", z);
//     let heart_eyed_cat = '😻';
//     println!("heart_eyed_cat: {}", heart_eyed_cat);
//  }

// ********** compound types **********

// what is compound data types?
// compound types can group multiple values into one type.
 
 //how many types of compound data types?
// there are two primitive compound types:
// 1. tuples
// 2. arrays

// what is tuples?
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.

// what is arrays data types?
// An array is a collection of multiple values that are the same type.
// Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.



// example of tuples

//  fn main (){
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("The value of y is: {}", y);
//     println!("The value of x is: {}", x);
//     println!("The value of z is: {}", z);
//  }

// example of tuples-2  
// accessing tuple elements directly by using a period (.) followed by the index of the value we want to access.

//  fn main (){
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = tup.0;
//     let six_point_four = tup.1;
//     let one = tup.2;
//     println!("The value of five_hundred is: {}", five_hundred);
//     println!("The value of six_point_four is: {}", six_point_four);
//     println!("The value of one is: {}", one);
//  }

// example of arrays

//  fn main (){
//     let a = [1, 2, 3, 4, 5];
//     println!("a: {:?}", a);
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//     "August", "September", "October", "November", "December"];
//     println!("months: {:?}", months);
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("a: {:?}", a);
//     let a = [3; 5];
//     println!("a: {:?}", a);
//  }

 // example of arrays-2
 //what is accessing array elements?
//  an array is a single chunk of memory allocated on the stack.
//  accessing array elements directly by using a period (.) followed by the index of the value we want to access.
// 

//  fn main (){
//     let a = [1, 2, 3, 4, 5];
//     let first = a[0];
//     let second = a[1];
//     println!("first: {}", first);
//     println!("second: {}", second);
//  }


// invalid array element access
 // what is invalid array element access?
//  if we try to access an element using indexing syntax, but we use an index value that is greater than or equal to the array length, Rust will prevent the code from compiling.
//  Rust will also prevent us from compiling if we attempt to access an array element using a floating-point number.

//  fn main (){
//     let a = [1, 2, 3, 4, 5];
//     let index = 10;
//     let element = a[index];
//     println!("The value of element is: {}", element);
//  }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

   
