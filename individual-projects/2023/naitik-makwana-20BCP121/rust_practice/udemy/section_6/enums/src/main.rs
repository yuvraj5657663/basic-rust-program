enum Post {
    Head,
    TeamLead,
    Senior,
    Junior

}


impl Post {
    fn salary(&self) -> u32 {

        let sal = match self {
            Post::Head => 2_00_00,
            Post::TeamLead => 1_00_000,
            Post::Senior => 70_000,
            Post::Junior => 30_000
        };
        sal
    }
}


//Using enum to build vector with diff datatypes 
#[derive(Debug)]
enum Types {
    Int(i32),
    Float(f32),
    Char(char),
    Bool(bool)
}


fn main() {

    let emp_1:Post = Post::Head;
    let emp_2:Post = Post::TeamLead;
    let emp_3:Post = Post::Senior;
    let emp_4:Post = Post::Junior;

    println!("The salary of emp_1 is : {}", emp_1.salary());
    println!("The salary of emp_2 is : {}", emp_2.salary());
    println!("The salary of emp_3 is : {}", emp_3.salary());
    println!("The salary of emp_4 is : {}", emp_4.salary());

    let diff_type_vec: Vec<Types> = vec![Types::Int(5), Types::Float(3.3), Types::Char('c'), Types::Bool(true)];

    println!("Vector of diff type : {:?}", diff_type_vec);   

    for i in diff_type_vec.iter() {
        match i {
            Types::Int(num) => println!("interger type : {}",num),
            Types::Float(num) => println!("float type : {}",num),
            Types::Char(ch) => println!("char type : {}",ch),
            Types::Bool(bool) => println!("boolean type : {}",bool),
        }
    } 
}
