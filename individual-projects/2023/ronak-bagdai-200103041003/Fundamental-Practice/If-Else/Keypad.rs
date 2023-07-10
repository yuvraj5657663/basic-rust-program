fn main() {
    let keyboard_type = "qwerty";
    
    if keyboard_type == "a-z" {
        println!("The keyboard does not have a numeric keypad.");
    } else if keyboard_type == "numeric" {
        println!("The keyboard has a numeric keypad.");
    } else {
        println!("Unknown keyboard type.");
    }
}
