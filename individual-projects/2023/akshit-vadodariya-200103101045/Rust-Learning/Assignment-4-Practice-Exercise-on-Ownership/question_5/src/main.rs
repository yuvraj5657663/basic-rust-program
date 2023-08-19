fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
    &num_vec[0]
}

fn main() {
    /* There is an issue with respect to borrowing rules in the code below. Try identifying it.

    fn main() {
        let mut some_vec = vec![1, 2, 3];
        let first = get_first_element(&some_vec);
        some_vec.push(4);
        println!("The first number is: {}", first);
    }

    fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
        &num_vec[0]
    } */

    let mut some_vec = vec![1,2,3];
    let first = get_first_element(&some_vec);
    println!("The first number is: {}", first);
    some_vec.push(4);
}
