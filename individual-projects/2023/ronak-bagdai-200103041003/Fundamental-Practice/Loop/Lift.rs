fn main() {
    let total_floors = 14;
    let mut current_floor = 1;
    let lift_position = 8;

    // Find the lift
    while current_floor != lift_position {
        println!("Current floor: {}", current_floor);
        current_floor += 1;
    }

    // Repair the lift
    println!("Found the lift at floor {}!", lift_position);
    println!("Repairing the lift...");
    println!("Going Down...↓↓↓");

    // Return to ground floor
    while current_floor != 1 {
        current_floor -= 1;
        println!("Current floor: {}", current_floor);
    }
    println!("You have completed the lift repair job!");
}
