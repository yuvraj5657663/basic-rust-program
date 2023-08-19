fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
    println!("{}", v[1]);

    let string_array = vec!["apple", "cheery", "grapes"];
    println!("{:?}", string_array);

    let mut string_array = vec!["My name"; 3];
    string_array[2] = "dhruv shah";
    println!("{:?}", string_array);

    let index = v.get(6);
    println!("{:?}", index);

    v.push(50);
    println!("the value is{:?}" , v);

    v.remove(2);
    println!("the value is{:?}" , v);

    println!("the value of 5 exist: {}" , v.contains(&5));
}
