fn main(){
    let total_students = 60;
    let mut present_students = 0;
    let mut absent_students = 0;
    let mut present = true;

    for i in 1..=total_students {
        println!("Is Student {} present? (yes/no)" ,i);

        if present {
            println!("The Student is present");
            present_students += 1;
        }
        else {
            print!("The Student is absent");
            absent_students += 1;
        }
    }
}