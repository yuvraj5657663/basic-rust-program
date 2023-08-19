fn main() {
    let _some_string = "Fixed length string";

    let mut _growable_string = String::from("Growable string");
    println!("The String is: \"{}\"", _growable_string);

    _growable_string.push('s');
    println!("The String is: \"{}\"", _growable_string);

    _growable_string.pop();
    println!("The String is: \"{}\"", _growable_string);

    _growable_string.push_str(" are cool");
    println!("The String is: \"{}\"", _growable_string);

    println!(
        "is_empty(): {}, length(): {}, Bytes(): {}, Contains cool: {}",
        _growable_string.is_empty(),
        _growable_string.len(),
        _growable_string.capacity(),
        _growable_string.contains("cool")
    );

    _growable_string.push_str("     ");
    let _str_len = _growable_string.trim().len();

}
