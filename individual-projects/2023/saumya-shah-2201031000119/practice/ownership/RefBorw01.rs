fn main() {
    let mut str = String::from("hello");
    change(&mut str);
    println!("{str}");
}

fn change(some_str: &mut String) {
    some_str.push_str(" world")
}