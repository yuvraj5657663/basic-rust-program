fn working() {
    return true;
}

fn main() {
    let c = number_of_switches; //user input switches.
    let working_switch = 0;
    let not_working_switch = 0;

    for i in 0..c {
        if i == working() {
            println!("It's a working");
            working_switch += 1;
        }
        else {
            println!("It's not working");
            not_working_switch += 1;
        }
    }

    println!("number of working swiches : {}", working_switch);
    println!("number of not working swiches : {}",not_working_switch);
}