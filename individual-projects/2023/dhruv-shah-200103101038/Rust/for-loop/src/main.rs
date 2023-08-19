fn main() {
    // example 1
    for _a in 1..=20 {
        println!("{}", _a)
    }

    //example 2
    let mut _vec = vec![45, 50, 25, 85, 39, 28];
    for i in &mut _vec {
        *i += 5;
        // '*i' is used to dereference the value of 'i' and then add 5 each element.
        // iter() is used to iterate over the elements of the vector.
        println!("{}", i);
    }

}
