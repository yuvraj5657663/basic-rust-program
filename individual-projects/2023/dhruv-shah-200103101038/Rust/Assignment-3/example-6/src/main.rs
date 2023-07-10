fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    let doubled = double(x);
    double(doubled)
}

fn main() {
    println!("For 1: the expected value is 4 while the output is {}", quadruple(1));
    println!("For 2: the expected value is 8 while the output is {}", quadruple(2));
    println!("For 3: the expected value is 12 while the output is {}", quadruple(3));
    println!("For 4: the expected value is 16 while the output is {}", quadruple(4));
}
