fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    /* In this question, your task is to refactor the code in the main function to remove all variables while preserving the same functionality. Your program should make use of the functions double() and triple() that are defined within the program.

    The goal is to rewrite the code in a way that achieves the same outcome as the original code, but without using any variables directly in the main function. Instead, you should rely on the double() and triple() functions to perform the necessary calculations and return the desired results.

    By appropriately utilizing these functions, you can simplify the code in the main function while achieving the same desired outcome.


    fn double(x: i32) -> i32 {
        x * 2
    }

    fn triple(x: i32) -> i32 {
        x * 3
    }

    fn main() {

        let x = triple(double(5));

        let y = triple(x);

        println!("Answer: {}", y);

    } */

    println!("Answer: {}", triple(triple(double(5))));
}
