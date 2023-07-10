fn expected_minutes_in_oven() -> u32 {
  40
}

fn remaining_minutes_in_oven(actual_minutes_in_oven: u32) -> u32 {
  expected_minutes_in_oven() - actual_minutes_in_oven
}

fn preparation_time_in_minutes(number_of_toppings: u32) -> u32 {
  2 * number_of_toppings
}

fn elapsed_time_in_minutes(number_of_toppings: u32, actual_minutes_in_oven: u32) -> u32 {
  preparation_time_in_minutes(number_of_toppings) + actual_minutes_in_oven
}

fn main(){
    let number_of_toppings = 2;
    let actual_minutes_in_oven = 30;
    
    println!("Pizza in oven: {} minutes", actual_minutes_in_oven);
    println!("Expected minutes in oven: {} minutes", expected_minutes_in_oven());
    println!("Remaining minutes in oven: {} minutes", remaining_minutes_in_oven(actual_minutes_in_oven));
    println!("Preparation time: {} minutes", preparation_time_in_minutes(number_of_toppings));
    println!("Elapsed time: {} minutes", elapsed_time_in_minutes(number_of_toppings, actual_minutes_in_oven));
}