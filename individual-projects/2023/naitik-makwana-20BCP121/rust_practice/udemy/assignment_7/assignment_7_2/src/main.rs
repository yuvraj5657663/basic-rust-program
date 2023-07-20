// Section-6 // Assignment-7

/* Problem - 2
For the code below defines a Person struct. In the main program, we created a vector that holds instances of Person and added several entries to it. Your task is to write a statement using iterators to collect all the ages of the different persons into a single vector.
*/

struct Person {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: i32,
}



fn main() {

    let mut persons: Vec<Person> = Vec::new();

    persons.push(Person {first_name: "Naitik".to_string(),last_name: Some("Makwana".to_string()),age: 21});

    persons.push(Person {first_name: "Rajan".to_string(),last_name: Some("Patel".to_string()),age: 22});

    persons.push(Person {first_name: "Yash".to_string(),last_name: None,age: 20});

    persons.push(Person {first_name: "Parthraj".to_string(),last_name:Some("Gohil".to_string()),age: 21});  

    let mut ages : Vec<i32> = Vec::new();
    for p in &persons {
        ages.push(p.age);
    }

    println!("{:?}",ages );
}