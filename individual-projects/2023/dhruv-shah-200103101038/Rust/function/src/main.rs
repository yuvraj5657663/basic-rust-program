fn main() {
    basic();
    _function_with_inputs("Dhruv", 50000);

    let _full_name = "Dhruv";
    let _salary_information = 50000;
    _function_with_inputs(_full_name, _salary_information);

    let _answer = _function_with_inputs_outputs(50, 20);
    println!("the answer is {}", _answer);

    println!("the answer is {}", _return_function(20));

    let (_multiplication, _addition, _subtraction, _division) =
        _function_with_inputs_multiple_outputs(30, 15);
    println!(
        "multiplication = {}, addition = {}, subtraction = {}, division = {}",
        _multiplication, _addition, _subtraction, _division
    );

    // how to get user input

    let mut _input = String::new();
    println!("enter your name:");
    std::io::stdin()
        .read_line(&mut _input)
        .expect("failed to read line");
    println!("your name is: {}", _input);
}

fn basic() {
    println!("this is simple function");
}

fn _function_with_inputs(_name: &str, _salary: i32) {
    println!("my name is {} and salary is {}", _name, _salary);
}
// return function are specified by dash , follow by sign (>)

fn _return_function(x: i32) -> i32 {
    x * 20
}

fn _function_with_inputs_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn _function_with_inputs_multiple_outputs(num1: i32, num2: i32) -> (i32, i32, i32, i32) {
    (num1 * num1, num1 + num2, num1 - num2, num1 / num2)
}

   