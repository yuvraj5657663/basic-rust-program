// merathon

fn main() {
    for n in 1..40 {
        if n == 10 {
            println!("their are 30km remaining continue your runnig");
        } else if n == 20 {
            println!("their are 20km remaining continue your runnig");
        } else if n == 30 {
            println!("their are 10km remaining continue your runnig");
        } else if n == 40 {
            println!("you are in the finish line");
        } else {
            println!("{}", n);
        }
    }
}