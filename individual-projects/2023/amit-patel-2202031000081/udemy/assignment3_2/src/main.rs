fn main() {



    println!("Answer: {}", triple(triple(double(5))));

}
fn double(x: i32) -> i32 {

    x * 2

}

fn triple(x: i32) -> i32 {

    x * 3

}


/*
Rewrite the main function in a way so that there is no variable in it and it performs the same job as this program. Your program should make calls to both the functions in this program.


fn double(x: i32) -> i32 {

    x * 2

}

fn triple(x: i32) -> i32 {

    x * 3

}

fn main() {

    let x = triple(double(5));

    let y = triple(x);

    println!("Answer: {}", y);

} */