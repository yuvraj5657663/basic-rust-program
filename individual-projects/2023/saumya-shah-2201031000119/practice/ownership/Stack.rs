// -----|
// Stack|
// -----|
const MAX_VALUE:i32 = 40_000;
fn main() {
    let l = 20;
    let h = 40;

    let area = area_of_square(l, h);
    println!("Area of square is : {}", area);
}

fn area_of_square(m1:i32, m2:i32) -> i32{
    let square = square(m1 + m2);
    square
}

fn square(sq:i32) -> i32 {
    sq * sq
}