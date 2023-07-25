fn stack_function(mut var:i32) -> i32 {
    var = 50;
    var
}

fn main() {
    /* What will be the value of the variable stack_num at the end of the main() function in the program given below.
    fn main() {
        let mut stack_num = 56;
        stack_num  = stack_function(stack_num);
        println!("The value of the stack_num after the function call is {} ", stack_num);
    }

    fn stack_function(mut var:i32) -> i32 {
        var = 50;
        var
    } */

    let mut stack_num = 56;
    stack_num = stack_function(stack_num);
    println!("The value of the stack_num after the function call is {}", stack_num); //50
}
