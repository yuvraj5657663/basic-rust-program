fn fibon(n: i32) -> i32{
    if n < 0 {
        println!("Incorrect value!");
    }
    else if n == 0 {
        return 0;
    }
    else if n==1 || n==2 {
        return 1;
    }
    else {
        return fibon(n-1)+ fibon(n-2);
    }
}

fn main() {
    let n = 4;
    for i in 0..n {
        println!(fibon(i));
    }
}


//step:1
// fibon(0)  ->  return 0

//step:2
//fibon(1)  ->  return 1

// step:3
// fibon(2)  -> return 1

// step:4
// fibon(3)  -> return fibon(2) + fibon(1) -> 1 + 1 -> 2
//                        |           |
//                        V           V
//                      return 1    return 1
//                        |           |
//                        V           V
//                        1           1
// step:5
// fibon(4)  -> return fibon(3)    +    fibon(2) -> 2 + 1 -> 3
//                         |               |
//                         V               V
//                fibon(2) + fibon(1)      return 1
//                   |          |             |
//                   V          V             |
//                return 1    return 1        |
//                   |          |             |
//                   V          V             V
//                   1    +     1      +      1