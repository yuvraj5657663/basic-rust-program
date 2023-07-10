//onvert temperatures between Fahrenheit and Celsius.

fn take_input () -> i32 {

    let mut op = String::new();
    std::io::stdin().read_line(&mut op).expect("please Enter numeric value");
    let op = op.trim().parse::<i32>().expect("Unable to read input");
    return op
} 

//Formula (32°F − 32) × 5/9 = 0°C
fn far_to_cel (far: i32) -> f32 {
    ((far as f32) - 32.0) * 5.0 / 9.0
}

//Formula (0°C × 9/5) + 32 = 32°F
fn cel_to_far (cel: i32) -> f32 {
    ((cel as f32)*9.0/5.0 ) + 32.0
}

fn main() {
    println!("Select any option : \n 1. Fahrenheit to Celsius \n 2. Celsius to Fahrenheit");
    let selector = take_input();
    
    match selector {
        1 => {
            println!("Enter temp in Fahrenheit : ");
            let temp = take_input();
            println!("temp in celsius is : {}", far_to_cel(temp));
        },
        2 => {
            println!("Enter temp in celsius : ");
            let temp = take_input();
            println!("temp in Fahrenheit is : {}", cel_to_far(temp));
        }
        _ => println!("Wrong selection"),
    }

}
