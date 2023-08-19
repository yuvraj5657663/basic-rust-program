fn main() {
    // Will the program below compile? If not, can you identify the error?
    // let x1 = 40;
    // let x2 = x1;
    // x2 = x1-2;

    // println!("My age is {} and my son age is {}", x1,x2);


    // in rust all variable immutable by default 
    // if try to change value of the variable we write "mut" keyword 

    // Right code:
    let x1 = 40;
    let mut x2 = x1;
    x2 = x1 - 2;

    println!("My age is {} and my son age is {}", x1, x2);
}
