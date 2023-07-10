fn add(a: i32, b:i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b:i32) -> i32 {
    return a - b;
}

fn mul(a: i32, b:i32) -> i32 {
    return a * b;
}

fn div(a: i32, b:i32) -> i32 {
    return a / b;
}

fn main() {
    let sum = add(2,6);
    println!("Sum of two value is {}", sum);
    let sub = sub(6,2);
    println!("Sub of two value is {}", sub);
    let mul = mul(2,6);
    println!("Mul of two value is {}", mul);
    let div = div(6,2);
    println!("Div of two value is {}", div);
}