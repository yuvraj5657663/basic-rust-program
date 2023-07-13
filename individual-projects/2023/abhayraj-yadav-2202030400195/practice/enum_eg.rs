enum colours{
    blue,
    brown,
    black,
    red
}

fn which_colour(check : colours){
   
    match check {
        colours::blue =>     println!("SELECTED COLOUR IS BLUE."),
        colours::black =>     println!("SELECTED COLOUR IS BLACK. "),
        colours::brown =>     println!("SELECTED COLOUR IS BROWN."),
        colours::red =>     println!("SELECTED COLOUR IS RED.")
    }
}
fn main(){
    which_colour(colours::brown);
}