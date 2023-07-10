fn double(x: i32) -> i32 {
    x * 2
}
fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    println!("answer: {}", triple(triple(double(5))));
}
