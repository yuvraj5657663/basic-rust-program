fn main() {
    // example 1
    let mut var = 100;
    loop {
        var -= 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!("the highest number divide by 13 is {}", var);

    // example 2
    let mut var = 0;
    let mut count = 0;
    let req_num = loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("the number is divide by 3 and 5 is {}", var);
            count += 1;

            if count == 3 {
                break var;
            } else {
                continue;
            }
        }
        println!("{}", var);
    };
    println!("the third highest number is {}", req_num);

    //  example 3
    let mut number = 0;
    loop {
        number += 1;
        // condition to skip the iteration
        if number == 3 {
            continue;
        }
        // condition to exit the loop
        if number > 5 {
            break;
        }
        println!("{}", number);
    }

    //  example 4 for-loop for break and continue
    let mut _i = 0;
    for _i in 1..10 {
        if _i == 4 {
            break;
        } else if _i == 5 {
            continue;
        } else {
            println!("{}", _i);
        }
    }

    // example 5 while loop for break and continue
    let mut i = 0;
    while i < 10 {
        if i == 3 {
            break;
        } else if i == 5 {
            continue;
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}
