fn main() {
    let mut var = 0;
    let mut count = 0;

    loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("The number which is divisible by 3 and 5 is {}\n", var);
            count += 1;
            if count == 5 {
                break;
            } else {
                continue;
            }
        }
        println!("{}", var);
    }
}
