fn main() {
    let some_number = 102;
    match some_number {
        1 => println!("One"),
        2 => println!("Two"),
        3..=99 => println!("Three"),
        100 => println!("Hundred"),
        _ => println!("Something else"),
    }

    let marks = 50;
    let mut grade = 'N';
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    }
    println!("Grade is {}", grade);

    let marks = 90;
    let mut grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("Grade is {}", grade);
}
