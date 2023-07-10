fn working() {
    return true;
}

fn remove_pen() {
    //remove the pen in box
}

fn main() {
    let p = number_of_pens; // user input
    let working_pen = 0;
    let not_working_pen = 0;

    for i in 0..p {
        if i == working() {
            working_pen +=1;
        }
        else {
            remove_pen();
        }
    }

    println!("working pen are : {}", working_pen);
}