#[derive(Debug)]
struct Employee {

    emp_id : u32,
    name: String,
    age : u32,
    gender : char,
    post : String,
    salary : u32
}

impl Employee {

    fn new() -> Self { // to create instance of type struct with default values
        Employee /*Self */ {
            emp_id : 0002,
            name : String::from("Rajan"),
            age : 21,
            gender : 'M',
            post : String::from("Designer"),
            salary : 100000
        }
    }

    fn tax (&self) -> f32 {
        (self.salary as f32)*5.0/100.0
    }
}

//Tuples Struts
struct Point(i32,i32) ;

impl Point {
    fn distance_from_origin(&self) -> f32 {
        ((self.0 as f32).powf(2.0) + (self.1 as f32).powf(2.0)).sqrt()
    }
}


fn main() {

    let emp_1 : Employee = Employee {
        emp_id : 0001,
        name : String::from("Naitik"),
        age : 21,
        gender : 'M',
        post : String::from("intern"),
        salary : 10000
    };

    println!("the basic info of employee 1 is name : {} and age : {}",emp_1.name, emp_1.age );

    println!("amount of tax to be paid : {} ", emp_1.tax()); // can be written as Employee::tax(&emp_1)

    let emp_2 = Employee::new(); // creating struct instance with default values
    println!("{:?}", emp_2 );

    let emp_3 = Employee {
        emp_id : 0003,
        name : String::from("Ronak"),
        ..emp_1
    }; // creating struct instance by providing some info and rest derive from another struct instance
    println!("{:?}", emp_3);

    let mut emp_4 = Employee::new(); // mutable structure instance
    println!("employee id : {} , name : {} , post : {}", emp_4.emp_id, emp_4.name, emp_4.post); 

    emp_4.emp_id = 0004;
    emp_4.name = String::from("Akshit");
    emp_4.post = String::from("Developer");
    println!("{:?}", emp_4);

    let p1 = Point(2,3);
    println!("x:{} , y:{}", p1.0,p1.1);
    println!("Distance from origin : {}", p1.distance_from_origin());
}