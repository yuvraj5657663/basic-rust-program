// Section-5 // Assignment-6

/* Problem - 4
In this exercise, you will be working on creating a student management system using Rust. The system should allow you to store and retrieve student information based on their unique ID. First, you will create the necessary data structures.

Create a Student structure representing a student with the following fields:
id: An integer representing the unique ID of the student.
name: A string representing the name of the student.
grade: A string representing the grade of the student.

Next, create a StudentManager structure representing the student management containing a field of student: This will essentially be a hashmap where the key part will be an integer representing unique ID of student and the value part will be the complete details of the students contained in the student structure.

The StudentManager should implement the following methods:
new() -> Self: A constructor that initializes an empty student manager.

add_student(&mut self, student: Student) -> Result<(), String>: Adds a student to the manager. If the student's ID already exists, return an error message. Otherwise, add the student to the manager and return Ok.

get_student(&self, id: i32) -> Option<&Student>: Retrieves a student from the manager based on their ID. If the student is found, return Some(student). Otherwise, return None.

Your task is to implement the Student structure, StudentManager structure, and the mentioned methods. Additionally, provide a sample usage of the student management system by adding a few students and retrieving their information using the get_student() method.
*/

use std::collections::HashMap;
#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    grade : char
}

struct StudentManager {
    student_map : HashMap<u32,Student>,   
}

impl StudentManager {
    fn new() ->Self {
        Self{
            student_map: HashMap::new()
        }
    }

    fn add_student(&mut self, student:Student) -> Result<(),String> {
        
        if self.student_map.contains_key(&student.id){
            Err("This student id already exists".to_string())
        }
        else {
            self.student_map.insert(student.id, student);
            Ok(())
        }
    }

    fn get_student(&self,id:i32) -> Option<&Student> {
        self.student_map.get(&(id as u32))
    }

}

fn main(){
    let s1 = Student {
        id : 1,
        name : String::from("Naitik"),
        grade : 'A'
    };

    let s2 = Student {
        id : 2,
        name : String::from("Rajan"),
        grade : 'A'
    };

    let mut sm1 =  StudentManager::new();
    sm1.add_student(s1);
    println!("{:?}", sm1.get_student(1));
    sm1.add_student(s2);
    println!("{:?}", sm1.get_student(2));
    println!("{:?}", sm1.get_student(3));

}