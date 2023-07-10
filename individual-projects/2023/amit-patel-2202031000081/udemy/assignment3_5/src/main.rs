
fn expected_minutes_in_oven()-> u32{
    return 40;
}
fn remaining_minutes_in_oven(passed_time:u32) ->  u32{
    expected_minutes_in_oven() - passed_time
}
fn preparation_time_in_minutes(layers:u32)-> u32 {
    layers*2

}
fn elapsed_time_in_minutes( topping:u32,time:u32)->u32{
   preparation_time_in_minutes(topping) + remaining_minutes_in_oven(time)

}

fn main() {
    println!("remining time for pizza:{} minutes.",elapsed_time_in_minutes(3,20));
    
}
/*
In this exercise question, we will be writting some code that will help us cook an awesome pizza from a given cooking book.

You are required to write four functions corresponding to some tasks. Some of the tasks may be related to each other also.

Function 1.

Write a function called expected_minutes_in_oven that will return a value which indicates how many minutes the pizza should be in the oven. According to the cooking book, the expected oven time in minutes is 40: Below is the skeleton of the function indicating what is expected from the function to return. 

expected_minutes_in_oven()

// Returns: 40



Function 2

Write a function called remaining_minutes_in_oven that takes the actual minutes the pizza has been in the oven as an input  parameter and returns how many minutes the pizza  still has to remain in the oven, based on the expected oven time in minutes from the previous task.

remaining_minutes_in_oven(30)

// Returns: 10 becuase 40- 30 = 10.

Function 3

Write a function called preparation_time_in_minutes that takes the number of toppings you added to the pizza as a parameter and returns how many minutes you spent preparing the pizza, assuming each layer takes you 2 minutes to prepare.

preparation_time_in_minutes(2)

// Returns: 4



Function 4

Write a function called elapsed_time_in_minutes that takes two input parameters: the first parameter is the number of toppings you added to the pizza, and the second parameter is the number of minutes the pizza has been in the oven. The function should return how many minutes you've worked on cooking the pizza, which is the sum of the preparation time in minutes, and the time in minutes the pizza  has spent in the oven at the moment.

elapsed_time_in_minutes(3, 20)

// Returns: 26

pub fn expected_minutes_in_oven() -> i32 {
    40
}
pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
        return (expected_minutes_in_oven() - actual_minutes_in_oven);
}
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
        return (number_of_layers * 2);
}
pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
        return (preparation_time_in_minutes(number_of_layers)+ actual_minutes_in_oven);
} */
// fn expected_minutes_in_oven () -> u32 {
//     return 40;
// }

// fn remaining_minutes_in_oven(x:u32) -> u32 {
//     expected_minutes_in_oven()-x
// }

// fn preparation_time_in_minutes (x:u32) -> u32 {
//     2*x
// }

// fn elapsed_time_in_minutes(topping:u32, spent:u32) -> u32 {
//     remaining_minutes_in_oven(spent) + preparation_time_in_minutes(topping)
// }

// fn main() {
//    print!("time remaining : {}", elapsed_time_in_minutes(5,20));
// }