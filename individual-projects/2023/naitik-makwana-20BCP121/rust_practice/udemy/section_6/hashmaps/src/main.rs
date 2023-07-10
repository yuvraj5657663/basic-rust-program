use std::{collections::HashMap, hash::Hash};

// fn main() {
//     let mut employee:HashMap<u32,&str> = HashMap::new();
//     employee.insert(1,"Naitik");
//     employee.insert(2,"Rajan");
//     employee.insert(3,"Ronak");
//     employee.insert(4,"Akshit");
//     employee.insert(5,"Dhruv");

//     let id = 6;
//     // println!("Employee id {} is given to : {:?}",id, employee.get(&id).unwrap());

//     if employee.contains_key(&id) {
//         println!("yes key is present");
//     }else {
//         println!("key is not present");
//     }

//     match employee.get(&id) {
//         Some(val) => println!("value exist {}", val),
//         None => println!("value doesnt exists"),
//     }

//     for (&e_id,&name) in &employee {
//         println!("the employee id is {} and name is : {}",e_id, name);
//     }

//     employee.insert(6,"Parva");
//     //to check if key has associated value if not then assign and if present keep as it is
//     employee.entry(6).or_insert("Raj");

//     println!("the employees : {:?}", employee);
// }

fn main() {
    let my_vec = vec![1,2,3,4,5,2,3,4,5,3,4,5,4,5,5];

    let mut freq : HashMap<u32,u32> = HashMap::new();

    for i in &my_vec{
        let f:&mut u32 = freq.entry(*i).or_insert(0);
        println!("value of f is : {}",*f);
        *f+=1
    }

    println!("{:?}", freq);
}