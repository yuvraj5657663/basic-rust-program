const PI: f32 = 3.14;
fn main() {
    let val = 101-3; // Change this value to see the output

    let var = match val {
        1..=100 => "Between 1-100",
        101..=200 => "Between 101-200",
        _ => "Something else",
    };
    println!("{var}");
    Pi_func();

    String_func();
}
fn Pi_func() {
    println!("Value of pi : {PI}");
}

fn String_func() {
    let str = String::from("HelloGoodMorning");
    println!("String : {}", str);
}