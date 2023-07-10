fn main() {
    // loop {
    //     println!("Dhruv");
    // }

    // example 1
    let mut a = 1;
    while a <= 10 {
        println!("{}", a);
        a += 1;
    }

    // example 2
    let _my_number = 10;
    let mut _guess = false;

    println!("guess the number which is boolean 1 and 20");

    while _guess != true {
        let mut line = String::new();
        println!("Enter Here");
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        let _x: i32 = line.trim().parse().expect("Input not an integer");

        if _my_number == _x {
            println!("your guess the number correct!");
            _guess = true;
        } else {
            println!("please try again")
        }
    }

    // example 3
    println!("enter a number and i will tell you the next number after your number that is divisible by both 2 and 5");

    let mut line = String::new();
    println!("Enter Here");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();

    let mut _x: i32 = line.trim().parse().expect("Input not an integer");
    let mut _divisible_by_2_5 = false;

    _x = _x + 1;
    while (_x % 2 == 0 && _x % 5 == 0) != true {
        _x = _x + 1;
    }
    println!("the number divide by both 2 and 5 is {}", _x);
}
