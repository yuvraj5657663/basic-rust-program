fn main() {
    let mut num_not_working = 0; 

    let mut basement1 = true;
    let mut basement2 = false;
    let mut basement3 = false;

    if !basement1 {
        println!("Camera 1 in Basement 1 is not working");
        num_not_working += 1;
    }
    if !basement2 {
        println!("Camera 2 in Basement 2 is not working");
        num_not_working += 1;
    }
    if !basement3 {
        println!("Camera 3 in Basement 3 is not working");
        num_not_working += 1;
    }
    
    if num_not_working == 1 {
        println!("Warning: 1 camera is not working.");
    } else if num_not_working > 1 {
        println!("Error: {} cameras are not working.", num_not_working);
    }
}
