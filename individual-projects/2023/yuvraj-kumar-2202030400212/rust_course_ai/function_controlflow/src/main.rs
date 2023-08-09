// function of syntax 
// fn function_name(parameters1: Type1, parameters2: type2,....)->ReturnType {
//     function body
//     code logic
//     optional return statement
// }

// example of simple fucntion
// fn add(a:i32, b:i32) -> i32 {
//     let sum = a + b;
//     sum
// }
// fn main(){
//     let result = add(5,3);
//     println!("sum:{}",result);
// }



// control flow 
//example of if else 
// fn main(){
//     let num = 10;
//     if num < 10 {
//         println!("number is greater than 10 ");

//     }else{
//         println!("number is not greater than 10");

//     }
// }



// example of loop ;

// fn main(){
//     let mut num = 0;
//     loop {
//         println!("num : {} ", num);
//         num += 3;
        
//         if num > 30 {
//             break;
//         }
//     }
// }


// example of while loop 
// fn main(){
//     let mut count = 0;
//     while count < 20 {
//         println!("count: {} ", count);
//         count += 2;
//     }
// }


// example of for loop 

fn main() {
    for num in 2..=20 {
        println!("number: {}",num);
    }
}
