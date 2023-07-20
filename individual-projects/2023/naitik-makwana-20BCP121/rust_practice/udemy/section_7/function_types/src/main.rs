/* Function pointer types, written using the fn keyword, refer to a function whose identity is not necessarily known at compile-time. They can be created via a coercion from both function items and non-capturing closures. */

// fn max(x:i32, y:i32) -> i32 {
//     if x>y {x} else {y}
// }

// fn min (x:i32, y:i32) -> i32 {
//     if x<y {x} else {y}
// }

// fn main() {
//     let mut fmin = min;
//     println!("minimum is : {}",fmin(8,4));

//     let mut fmax = max;
//     println!("minimum is : {}",fmax(8,4));


// }

/********************************************************/

// fn print_name(name:&str) {
//     println!("name : {}", name);
// }

// fn print_info(f: fn(&str), some_str:&str, age:u32) {
//     f(some_str);
//     println!("age : {}", age)
// }


// fn main(){
//     let my_name = "Naitik";
//     let my_age = 21;
//     print_info(print_name, my_name, my_age );
// }

/********************************************************/

fn add1 (x:i32) -> i32 {
    x+1
}

fn add_twice(f: fn(i32) -> i32, x:i32) -> i32 {
    f(x) + f(x)
}

fn main() {
    let ans = add_twice(add1, 10);
    println!("{}", ans);
}