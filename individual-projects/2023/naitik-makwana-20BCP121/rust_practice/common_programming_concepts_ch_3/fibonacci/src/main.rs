fn take_input () -> u32 {

    let mut op = String::new();
    std::io::stdin().read_line(&mut op).expect("please Enter numeric value");
    let op = op.trim().parse::<u32>().expect("Unable to read input");
    return op
}

fn fibonacci(n:u32) -> u32 {
    
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2), 
    }


} 

fn main() {
    println!("Enter number of term to print of fibonacci : ");
    let n = take_input();

    println!("Here are the first {} of fibonacci series :",n );
    for i in 1..=n {
        println!("{}", fibonacci(i));
    }
}
