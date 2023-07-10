// switches is working or not

fn main(){
    let c = 50;
    let switch = on;
    let working_switch = 0;
    let not_working_switch = 0;

    for switch in 0..c {
        if switch == on {
            println!("the switch is working");
            working_switch ++;
        }
        else if switch == off {
            println!("the switch is not working");
            not_working_switch ++;
        }
        else{

        }
    }

    println!("the total number of working switches is {}", working_switch);
    println!("the total number of not working switches is {}", not_working_switch);
}