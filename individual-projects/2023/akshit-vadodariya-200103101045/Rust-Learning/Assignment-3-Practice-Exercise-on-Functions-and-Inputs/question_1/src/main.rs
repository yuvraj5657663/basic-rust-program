fn add_3(x: i32) -> i32 {
    x + 3
}

fn add_5(y: i32) -> i32 {
    y + 5
}

fn times(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    /* In this exercise, your task is to define and implement three functions used in the program provided below. The details of each function are as follows:

    1. add_3(x): This function should add three to the input variable ‘x’ and the return the resultant value.

    2. add_5(x): This function should add five to the input variable ‘x’ and the return the resultant value.

    3. times(x,y): This function should compute the multiplication of the inputs ‘x’ and ‘y’ and return the resultant value.



    fn main() {

            let x = 3;

            let y = 4;

             println!(

                    "The result of x+3 times y+5 is {}",

                    times(add_3(x), add_5(y))

            );

    } */

    let x = 3;
    let y = 4;
    println!(
        "The result of x + 3 times y + 5 is {}",
        times(add_3(x), add_5(y))
    );
}
