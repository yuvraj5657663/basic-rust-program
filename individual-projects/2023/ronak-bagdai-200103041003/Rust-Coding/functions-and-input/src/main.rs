fn main() {
    basic_fn();
    function_with_inputs("Ronak", 100000);

    let full_name = "Ronak Bagdai";
    let salary_info = 100000;
    function_with_inputs(full_name, salary_info);

    let c = multiplication(10, 20);
    println!("Multiplication of 10 and 20 is {}", c);

    let (d, e, f) = multiplication_with_multiple_output(10, 20);
    println!(
        "Multiplication = {}, Addition = {}, Subtraction = {}",
        d, e, f
    );

    let result = multiplication_with_multiple_output(10, 20);
    println!(
        "Multiplication = {}, Addition = {}, Subtraction = {}",
        result.0, result.1, result.2
    );

    let full_name = {
        let first_name = "Ronak";
        let last_name = "Bagdai";
        format!("{} {}", first_name, last_name)
    };
    println!("Full name is {}", full_name);

    let mut n = String::new();
    println!("Enter a number: ");
    std::io::stdin().read_line(&mut n).expect("Failed to read line");

    let n:char = n.trim().parse().expect("Please type a number");
    print!("You entered {}", n);
}

fn basic_fn() {
    println!("Hello, world!");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("{} has a salary of {}", name, salary);
}

fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

fn multiplication_with_multiple_output(a: i32, b: i32) -> (i32, i32, i32) {
    (a * b, a + b, a - b)
}
