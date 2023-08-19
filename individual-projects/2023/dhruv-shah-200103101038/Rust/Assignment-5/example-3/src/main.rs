// This question is about writing some code to analyze the production of an assembly line in a car factory.
// The assembly line have different speeds which can range from 0 (off) to 10 (maximum).
//  At its lowest speed that is the 1, a total of 221 cars are produced each hour.
//  The fproduction increases linearly with the speed. This means that when the speed is set to 4,
//  it should produce 4 * 221 = 884 cars per hour.
//   However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded.
//   The following table shows how speed influences the success rate:

// 1 to 4: 100% success rate.
// 5 to 8: 90% success rate.
// 9 and 10: 77% success rate.

// You are requied to write two functions for the following two scenarios.

// 1. Write a function called total_production() which will calculate the assembly line's total production
//  in some specified time given in hours,taking into account its success rate.
//   The input to the function will be the number of hours and speed while the output will
//    be the number of cars successfully produced without the faults.

// 2. Write another function called Cars_produced_per_minutes().
//  The input to the function will be the hours and speed while the output wil be
//  the number of cars successfully produced per minutes.

fn main() {
    println!("Enter the number of hours: ");
    let hours = read_input();
    println!("Enter the speed of the assembly line: ");
    let speed = read_input();
    let total_production_value = total_production(hours, speed);
    let cars_produced_per_minute_value = cars_produced_per_minute(hours, speed);
    println!("The total production is {} cars", total_production_value);
    println!(
        "The cars produced per minute is {} cars",
        cars_produced_per_minute_value
    );
}

fn total_production(hours: u32, speed: u32) -> u32 {
    let car_speed = 221 * speed;
    let total_car = match speed {
        1..=4 => car_speed,
        5..=8 => car_speed * 9 / 10,
        9..=10 => car_speed * 77 / 100,
        _ => 0,
    };
    total_car
}

fn cars_produced_per_minute(hours: u32, speed: u32) -> u32 {
    total_production(hours, speed) / 60
}

fn read_input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();
    number
}

