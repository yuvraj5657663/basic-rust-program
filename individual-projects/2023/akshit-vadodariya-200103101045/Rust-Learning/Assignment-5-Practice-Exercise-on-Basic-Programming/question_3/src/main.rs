/* This question involves writing code to analyze the production of an assembly line in a car factory. The assembly line has different speeds, ranging from 0 (off) to 10 (maximum). At the lowest speed of 1, the assembly line produces a total of 221 cars per hour. The production rate increases linearly with the speed, meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

However, higher speeds increase the likelihood of producing faulty cars that need to be discarded. The success rate depends on the speed, as shown in the table below:
· Speeds 1 to 4: 100% success rate.
· Speeds 5 to 8: 90% success rate.
· Speeds 9 and 10: 77% success rate.

You need to write two functions:
1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.

2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.

Write the code for both functions based on the provided specifications. */

use std::io;

fn total_production(_hours: i32, _speed: i32) {
    let cars: i32;
    if _speed >= 1 && _speed <= 4 {
        cars = _hours * 221;
        println!(
            "The number of cars successfully produced {} cars with speed {}",
            cars, _speed
        );
    } else if _speed >= 5 && _speed <= 8 {
        cars = (_hours * 221 * 90) / 100;
        println!(
            "The number of cars successfully produced {} cars with speed {}",
            cars, _speed
        );
    } else if _speed >= 9 && _speed <= 10 {
        cars = (_hours * 221 * 77) / 100;
        println!(
            "The number of cars successfully produced {} cars with speed {}",
            cars, _speed
        );
    } else {
        println!("Invalid speed!")
    }
}

fn cars_produced_per_minute(_hours: i32, _speed: i32) {
    let minutes = _hours * 60;
    let cars: i32 ;
    let car:i32 ;

    if _speed >= 1 && _speed <= 4 {
        cars = _hours * 221;
        car = cars / minutes;
        println!(
            "The number of cars successfully produced {} cars with speed {}",
            car, _speed
        );
    } else if _speed >= 5 && _speed <= 8 {
        cars = (_hours * 221 * 90) / 100;
        car = cars / minutes;
        println!(
            "The number of cars successfully produced {} cars with speed {}",
            car, _speed
        );
    } else if _speed >= 9 && _speed <= 10 {
        cars = (_hours * 221 * 77) / 100;
        car = cars / minutes;
        println!(
            "The number of cars successfully produced {} cars with speed {}",
            car, _speed
        );
    } else {
        println!("Invalid speed!")
    }
}

fn main() {
    println!("Enter the time in hours: ");
    let mut time = String::new();
    io::stdin()
        .read_line(&mut time)
        .expect("Failed to read line");
    let time: i32 = time.trim().parse().expect("Please type a number!");

    println!("Enter the speed between 1 to 10: ");
    let mut speed = String::new();
    io::stdin()
        .read_line(&mut speed)
        .expect("Failed to read line");
    let speed: i32 = speed.trim().parse().expect("Please type a number!");

    total_production(time, speed);
    cars_produced_per_minute(time, speed);
}
