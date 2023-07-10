fn festival() {
    return true;
}

fn main() {
    let festival_date = user_input;

    if festival() {
        festival_date = festival_date - 1;
        println!(festival_date, " {} Wear a festival oriented cloths!");
    }
    else{
        println!("Not festival avalable!");
    }
}