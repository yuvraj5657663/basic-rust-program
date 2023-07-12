struct rectangle {
    width: u32,
    length: u32,
}
fn main() {
    let rect1 = rectangle {
        width: 30,
        length: 40,
    };
    println!("Area of rectanlge : {}", area(&rect1))
}

fn area (Rectangle: &rectangle) -> u32 {
    Rectangle.width * Rectangle.length
}