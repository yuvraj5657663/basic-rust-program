
// enum Option<T> {
//     None,
//     Some(T)
// }

// fn main() {
//     let mut car:Option<String> = None;

//     car = Some(String::from("Alto"));

//     match car {
//         Some(car_name) => println!("car is : {}",car_name),
//         None => println!("No car"),
//     }
    
// }

struct Person {
    name:String,
    age:u32
}

fn square (num:Option<i32>)->Option<i32> {
    match num {
        Some(number) => Some(number*number),
        None => None
    }
}

fn main(){
    let v1:Option<&str> = Some("Hello");
    println!("{:?} , {:?}", v1, v1.unwrap());


    let v2: Option<f64> = Some(1.1);
    let mut v3 = 2.2;
    v3 = v3 + v2.unwrap();
    println!("{}",v3);


    let v4:Option<Vec<i32>> = Some(vec![1,2,3]);
    println!("{:?}", v4.unwrap());

    let v5:Person = Person {
        name: String::from("Naitik"),
        age: 18
    };

    let some_person:Option<Person> = Some(v5);

    let number:Option<i32> = Some(6);

    if square(number) != None {
        println!("square is : {:?}", square(number).unwrap());
    }else {
        println!("dont have any value");
    }

    square(None);

}