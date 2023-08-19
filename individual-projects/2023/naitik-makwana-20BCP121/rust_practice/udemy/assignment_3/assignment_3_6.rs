// Section-2 // Assignment-3

/*Problem - 6

Consider the program below. Modify the definition of the quadruple function below by calling the double function twice (this means that hte quadruple function should only make call to the double function and it should call it twice). The quadruple function should return 4 times the number that has been provided to it as an input
*/

fn double(x: i32) -> i32 {

    x * 2

}

fn quadruple(x: i32)-> i32 {

    double(double(x))

}

fn main() {

    println!("For 1: the expected value is 4 while the output is {}", quadruple(1));

    println!("For 2: the expected value is 8 while the output is {}", quadruple(2));

    println!("For 3: the expected value is 12 while the output is {}", quadruple(3));

    println!("For 4: the expected value is 16 while the output is {}", quadruple(4));

}