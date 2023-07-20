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

// fn div<F: Fn(f32) -> bool>(x:f32,y:f32,f:F){
//     if f(y) == true {
//         println!("answer is : {}", x/y);
//     }else {
//         println!("error");
//     }
// }
// fn main () {
//     let div_stats = |y:f32| {if y!=0.0 {true} else {false}};

//     div(5.0,10.0,div_stats);
// }

fn main() {

    let c1 = |x:u32| -> u32 {x+x};
    let c2 = |x| {x+x};
    let c3 = |x|x+x ;

    println!("{}",c1(1));
    println!("{}",c2(1.1));
    println!("{}",c3(1));

    let mut vec1 = vec![1,2,3];

    let c4 = || {
        println!("vector : {:?}",vec1);
    };

    println!("{:?}",vec1 );
    c4();

    let mut vec2 = vec![4,5,6];

    let mut c5 = || {
        vec2.push(7);
    };
    c5();
    println!("{:?}",vec2 );

    let mut vec3 = vec![7,8,9];

    let mut c6 = || {
        let vec4 = vec3;
        println!("value of vec 3 transferred herea at vec 4 : {:?}",vec4 );
    };
    c6();

}
