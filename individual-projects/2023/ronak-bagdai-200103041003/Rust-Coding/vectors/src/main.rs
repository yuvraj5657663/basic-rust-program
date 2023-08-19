fn main() {
    let mut _number_vec = vec![1, 2, 3, 4, 5];
    println!("The vector is: {}", _number_vec[2]);   
    println!("The vector is: {:?}", _number_vec);

    let array_with_same_values = vec![0; 10];

    let mut string_array_1 = vec!["Hello", "World"];

    let string_array_2 = vec!["unknown"; 10];

    string_array_1[0] = "Hi";
    
    println!("The vector is: {:?}", string_array_1);

    let char_vec = vec!['a', 'b', 'c', 'd', 'e'];

    let sub_vec = &&_number_vec[1..3];

    println!("The subset vector is: {:?}", sub_vec);

    println!("Elements in the vector: {}", _number_vec.len());

    let check_index = _number_vec.get(100);
    println!("The element at index 100 is: {:?}", check_index);

    _number_vec.push(6);
    _number_vec.push(7);
    println!("The vector is: {:?}", _number_vec);

    _number_vec.remove(4);
    println!("The vector is: {:?}", _number_vec);

    println!("The value of 3 exist: {}", _number_vec.contains(&3));
    println!("The value of 3 exist: {}", _number_vec.contains(&5));
}
