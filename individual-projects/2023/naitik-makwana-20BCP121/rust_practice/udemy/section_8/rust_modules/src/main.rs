use rust_modules;

mod crate_1;

struct Rect {
    length : i32,
    width : i32
}

fn main() {
    rust_modules::extra_crate_1::printing();
    rust_modules::extra_crate_2::printing();

    let r1 = Rect {
        length : 4,
        width : 5
    };

    let r1_area = crate_1::rectangle_area(&r1.length, &r1.width);
    println!("The area of rectankgle is : {}", r1_area)
}
