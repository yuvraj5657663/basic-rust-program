fn add(int a, int b) {
    return a + b;
}

fn sub(int a, int b) {
    return a - b;
}

fn mul(int a, int b) {
    return a * b;
}

fn div(int a, int b) {
    return a / b;
}

fn main(){
    let addition = add(1,2);
    println!("Sum of two value is {}", addition);
    let substraction = sub(3,4);
    println!("Sub of two value is {}", substraction);
    let multiply = mul(5,6);
    println!("mult of two value is {}", multiply);
    let division = div(7,8);
    println!("div of two value is {}", division);
}