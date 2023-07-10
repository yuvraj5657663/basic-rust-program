// Section-4 // Assignment-5

/* Problem - 3
This question is about writing some code to analyze the production of an assembly line in a car factory. The assembly line have different speeds which can range from 0 (off) to 10 (maximum).  At its lowest speed that is the 1, a total of 221 cars are produced each hour. The production increases linearly with the speed. This means that when the speed is set to 4, it should produce 4 * 221 = 884 cars per hour. However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded. The following table shows how speed influences the success rate:
1 to 4: 100% success rate.
5 to 8: 90% success rate.
9 and 10: 77% success rate.
You are requied to write two functions for the following two scenarios.

1. Write a function called total_production() which will calculate the assembly line's total production in some specified time given in hours, taking into account its success rate. The input to the function will be the number of hours and speed while the output will be the number of cars successfully produced without the faults.

2. Write another function called Cars_produced_per_minutes(). The input to the function will be the hours and speed while the output wil be the number of cars successfully produced per minutes.
*/
static CAR_PR:u32 = 221;
fn main() {


    println!("Enter required time : ");
    let mut time = String::new();
    std::io::stdin().read_line(&mut time).expect("failure");
    let time:u32 = time.trim().parse().unwrap();

    println!("Enter required speed : ");
    let mut speed = String::new();
    std::io::stdin().read_line(&mut speed).expect("failure");
    let speed:u32 = speed.trim().parse().unwrap();

    println!("cars produced per hour without fault = {}", total_production(time,speed) );

    println!("cars produced per minutes without fault = {}", cars_produced_per_minutes(time,speed) );

}

fn total_production (time: u32, speed: u32) -> u32 {
    let total_car: u32 = match speed {
           
            1..=4 => {
                speed*time*CAR_PR
            },
            5..=8 => {
                (speed*time*CAR_PR*90)/100
            },
            9..=10 => {
                (speed*time*CAR_PR*77)/100
            },
            _ => {
                0
            }
    };
    total_car

}

fn cars_produced_per_minutes(time: u32, speed: u32) -> u32 {
    total_production(time,speed)/60
}