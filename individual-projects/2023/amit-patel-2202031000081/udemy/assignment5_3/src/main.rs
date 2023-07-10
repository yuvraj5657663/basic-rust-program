


fn main() {

    // input for hour
    
    println!("Enter current hour:");
    let mut current_hour = String::new();
    std::io::stdin()
        .read_line(&mut current_hour)
        .expect("Failed to read input");
    let current_hour: u32 = current_hour
        .trim()
        .parse()
        .expect("Invalid input for current hour");

    //input for speed

    println!("Enter current speed:");
    let mut current_speed = String::new();
    std::io::stdin()
        .read_line(&mut current_speed)
        .expect("Failed to read input");
    let current_speed: u32 = current_speed
        .trim()
        .parse()
        .expect("Invalid input for current speed");

    total_production(current_hour,current_speed);

    production_per_minute(current_hour,current_speed);

    
}
 fn total_production(n:u32,m:u32) {
    let production = match (n,m) {
        (1..=24, 1..=4) => n * 221,
        (1..=24, 5..=8) => n * (221 * 90 / 100),
        (1..=24, 9..=10) => n * (221 * 77 / 100),
        _ => {
            println!("Enter valid input");
            0
        }
    };

    println!("Successfully produced cars in the production house: {}", production);
}
fn production_per_minute(n:u32,m:u32){
    let production = match (n,m) {
        (1..=24, 1..=4) => n * 221/n*60,
        (1..=24, 5..=8) => n * (221 * 90 / 100)/60,
        (1..=24, 9..=10) => n * (221 * 77 / 100)/ 60,
        _ => {
            println!("Enter valid input");
            0
        }
    };

    println!("Successfully produced cars per minute is : {}", production);
}


/*
This question is about writing some code to analyze the production of an assembly line in a car factory. The assembly line have different speeds which can range from 0 (off) to 10 (maximum).  At its lowest speed that is the 1, a total of 221 cars are produced each hour. The production increases linearly with the speed. This means that when the speed is set to 4, it should produce 4 * 221 = 884 cars per hour. However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded. The following table shows how speed influences the success rate:

1 to 4: 100% success rate.

5 to 8: 90% success rate.

9 and 10: 77% success rate.

You are requied to write two functions for the following two scenarios.

1. Write a function called total_production() which will calculate the assembly line's total production in some specified time given in hours, taking into account its success rate. The input to the function will be the number of hours and speed while the output will be the number of cars successfully produced without the faults.



2. Write another function called Cars_produced_per_minutes(). The input to the function will be the hours and speed while the output wil be the number of cars successfully produced per minutes. */

