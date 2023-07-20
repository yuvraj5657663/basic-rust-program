// Section-5 // Assignment-6

/* Problem - 2
Consider a Student structure with three fields as shown below:
struct Student {
    name: String,
    age: i32,
    grade: String,
}

Create a hashmap called student_database to store instances of the Student structure. The keys of the hashmap should be unique student IDs, represented as integers.

Your task is to implement a function called add_student() that takes the student's ID, name, age, and grade as parameters. The function should add a new entry to the student_database hashmap if the student ID doesn't already exist. If the student ID already exists in the hashmap, the function should not add a new entry.
*/

use std::collections::HashMap;



struct Student {
    name: String,
    age: i32,
    grade: String,
}

fn add_student(student_database: &mut HashMap<i32, Student>,id: i32,name: String,age: i32,grade: String) {

    student_database.entry(id).or_insert(Student{name:name, age:age, grade:grade});

}


fn main() {

    let mut student_database: HashMap<i32, Student> = HashMap::new();

    add_student(&mut student_database,1,String::from("Naitik"),17,String::from("Grade 11"));

    add_student(&mut student_database,2,String::from("Rajan"),18,String::from("Grade 12"));

    add_student(&mut student_database,3,String::from("Yash"),16,String::from("Grade 10"));
    
    add_student(&mut student_database,2,String::from("Rajan"),18,String::from("Grade 12"));

    // Printing the student database
    for (id, student) in &student_database {

        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("++++++++++++++++++++++");

    }

}