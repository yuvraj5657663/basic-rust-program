fn main() {
    println!("Hello world");
}

//[1] fn main() {
//     let s1: String = String::from("this is me");
//     let s2: &str = "myself";

//     some_function(&s1, s2);
//     println!("{} {}", s1, s2);
// }

// fn some_function(a1: &String, a2: &str) {
//     println!("{} {}", a1, a2);
// }

// [2]  ans : v1

// [3] ans : v2

// [4]  ans : 50

// [5]  fn main() {

//     let mut some_vec = vec![1, 2, 3];

//     let first = get_first_element(&some_vec);

//     some_vec.push(4);

//     println!("The first number is: {}", first);

// }



// fn get_first_element(num_vec: &Vec<i32>) -> i32 {

//     &num_vec[0]

// }

// [6]  struct Person {

//     name: String,

//     age: i32,

// }

// fn main() {

//     let mut person = Person {

//         name: String::from("Alice"),

//         age: 30,

//     };

//     change_person(&person);

//     println!("Name: {}", person.name);

// }



// fn change_person(person: mut &Person) {

//     person.name = String::from("Bob");

// }

// [7] What is the issue with the borrowing in the code below?

// fn main() {

//     let mut numbers = vec![1, 2, 3];

//     let slice = get_slice(&mut numbers);

//     numbers.push(4);

//     println!("Slice: {:?}", slice);

// }



// fn get_slice(numbers: &Vec<i32>) -> &[i32] {

//     &numbers[0..2]

// }
