fn add(a: i32,b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32,b: i32) -> i32 {
    return a - b;
}

fn mul(a: i32,b: i32) -> i32 {
    return a * b;
}

fn div(a: i32,b: i32) -> i32 {
    return a / b;
}


fn main(){
    let sum = add(3,5);
    print!("The Sum is {}", sum);
    let sub = sub(7,5);
    print!("The Sub is {}", sub);
    let mul = mul(3,5);
    print!("The Mul is {}", mul);
    let div = div(3,5);
    print!("The Div is {}", div);
}