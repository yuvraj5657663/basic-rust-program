// Create a simple calculator program in Rust
// that supports basic arithmetic operations as well as advanced functions
// such as square root. Do not create any UI.
//  Add multiple examples in console to validate code.

fn _user_input() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("enter number");
    let x: i32 = input.trim().parse().expect("failed");
    x
}

fn add(a1: i32, a2: i32) -> i32 {
    a1 + a2
}

fn sub(a1: i32, a2: i32) -> i32 {
    a1 - a2
}

fn mul(a1: i32, a2: i32) -> i32 {
    a1 * a2
}

fn div(a1: i32, a2: i32) -> f64 {
    (a1 as f64) / (a2 as f64)
}

fn modulus(a1: i32, a2: i32) -> i32 {
    a1 % a2
}

fn power(a1: i32, a2: i32) -> i32 {
    a1.pow(a2 as u32)
}

fn root(a1: i32) -> f64 {
    (a1 as f64).sqrt()
}
fn main() {
    loop {
        println!("choose the option \n 1.Addition \n 2.Substraction \n 3.Multiplication \n 4.Division \n 5.Modulus \n 6.power \n 7.square_root");
        println!("enter the option:");
        let _option = _user_input();

        match _option {
            1..=6 => {
                println!("enter first number:");
                let _a1 = _user_input();
                println!("enter second number:");
                let _a2 = _user_input();

                match _option {
                    1 => println!("Addition: {}", add(_a1, _a2)),
                    2 => println!("Substraction: {}", sub(_a1, _a2)),
                    3 => println!("Multiplication: {}", mul(_a1, _a2)),
                    4 => println!("Division: {}", div(_a1, _a2)),
                    5 => println!("Modulus: {}", modulus(_a1, _a2)),
                    6 => println!("Power: {}", power(_a1, _a2)),
                    _ => println!("invalid option"),
                }
            }
            7 => {
                println!("enter the number:");
                let _a1 = _user_input();
                println!("square_root:{}", root(_a1));
            }
            _ => {
                println!("invalid option");
            }
        }

        println!("do you continue again ? \n 1.yes \n 2. no");
        let _exit = _user_input();

        if _exit == 2 {
            println!("Thank you..");
            break;
        }
    }
}
