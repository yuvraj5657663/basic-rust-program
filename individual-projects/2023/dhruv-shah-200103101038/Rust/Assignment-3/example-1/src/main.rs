fn main() {
    let x = (5 + 3) * (6 + 4);
    let y = times(add_3(5), add_4(6));
    assert_eq!(x, y);
    println!("good job!");
}

fn add_3(x: i32) -> i32 {
    x + 3
}

fn add_4(x: i32) -> i32 {
    x + 4
}

fn times(x: i32, y: i32) -> i32 {
    x * y
}
