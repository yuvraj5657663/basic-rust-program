fn main() {
    // match statement

    // match value {
    //     posible value(s) => {statement to execute},
    //     posible value(s) => {statement to execute},
    //     posible value(s) => {statement to execute},

    //     _=> {default execution of some statement}
    // }

    // example 1
    let _country_code = 91;

    match _country_code {
        91 => println!("India"),
        1 => println!("USA"),
        44 => println!("UK"),
        55 => println!("Brazil"),
        _ => println!("Unknown"),
    }

    // example 2
    let score = 250;

    match score {
        0 => println!("No runs scored."),
        1..=199 => println!("Scored {} runs. Not bad!", score),
        200 => println!("Scored a double century! Amazing!"),
        201..=500 => println!("Scored {} runs. Great innings!", score),
        _ => println!("Scored over 500 runs. Unbelievable!"),
    }

    // example 3
    let marks = 90;
    let grade = match marks {
        80..=95 => 'A',
        70..=79 => 'B',
        60..=69 => 'C',
        _ => 'F',
    };
    println!("the grade achieved is {}", grade);

    // example 4
    let mut input = String::new();
    println!("Enter number");
    let choice = std::io::stdin().read_line(&mut input).unwrap();
    let _x: i32 = input.trim().parse().expect("Input not an integer");

    match choice {
        1 => println!("your choice is 1"),
        2 => println!("your choice is 2"),
        3 => println!("your choice is 3"),
        _ => println!("wrong choice"),
    }
}
