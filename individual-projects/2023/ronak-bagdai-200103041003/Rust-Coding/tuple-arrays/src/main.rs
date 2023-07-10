fn main() {

    /* Tuples */

    let my_tuple: (&str, i64) = ("Enrollment", 200103041003);
    println!("{} is {}", my_tuple.0, my_tuple.1);

    println!("Another way of printing tuple: {:?}", my_tuple);

    let (enrollment, enrollment_number) = my_tuple;

    let enrollment = "my_tuple.0";
    let enrollment_number = "my_tuple.1";

    let nested_tuple = (4,5.0,(3,2),"Hello");
    let element = nested_tuple.2.0;
    println!("The element is {}", element);

    let empty_tuple = ();
    println!("The empty tuple is {:?}", empty_tuple);


    /* Arrays */

    let mut my_array = [1,2,3,4,5];
    println!("The array is {:?}", my_array);
    println!("The first element is {}", my_array[0]);

    my_array[0] = 10;
    println!("The array is {:?}", my_array);

    let array_with_same_elements = [1; 5];

    let mut string_array_1 = ["Hello", "World"];
    println!("The string array is {:?}", string_array_1);
    let string_array_2 = ["Hello"; 5];
    println!("The string array is {:?}", string_array_2);
    string_array_1[0] = "Hi";
    println!("The string array is {:?}", string_array_1);

    let char_array = ['a', 'b', 'c', 'd', 'e'];

    let mut array_number_1 = [1,2,3,4,5];

    let sub_array = &array_number_1[0..3];
    println!("The sub array is {:?}", sub_array);

    println!("The length of the array is {}", array_number_1.len());

    println!("The array is occupied in {} bytes", std::mem::size_of_val(&array_number_1));

    let check_index = array_number_1.get(100);
    println!("The value at index 100 is {:?}", check_index);
}