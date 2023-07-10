// // closuresa are anonymous function and has no name

// fn main() {
//     let a : i32 = 10;
//     // let square = |num:i32| println!("the square is : {}", num*num );
//     // square(a);

//     // let b : i32 = 20;
//     // square(b);

//     // let cube = |num:i32| println!("the cube is : {}", num*num*num );
//     // let c : i32 = 3;
//     // cube(c);


//     let printer = |info:String, name:&str, age:u32| println!(" {} {} {}",info, name, age);
//     let infor = String::from("Details: ");
//     let p_name:String = String::from("Naitik");
//     let p_age = 21;

//     printer(infor, p_name.as_str(), p_age);

// }

// fn main(){
//     let square = |num| num*num;
//     let x = 5.5;
//     println!("{}",square(x));
    
//     //after first closures call compiler will infer the type of operator and will not allow types afterwords.


// }

//closure as parameter to function

fn div<F: Fn(f32) -> bool>(x:f32,y:f32,f:F){
    if f(y) == true {
        println!("answer is : {}", x/y);
    }else {
        println!("error");
    }
}
fn main () {
    let div_stats = |y:f32| {if y!=0.0 {true} else {false}};

    div(5.0,10.0,div_stats);
}