fn main() {
    println!("Enter the number in  turms:");
    let mut number=String::new();
    std::io::stdin()
    .read_line(&mut number)
    .expect("faild to read number!!");
let mut number:u32=number
.trim()
.parse()
.expect("faild to read");

for  i in 0..number{
    println!("{}",fibo(i));
}
}

fn fibo(num:u32)->u32{
    if num<=1{
        num

    }
    else{
        fibo(num - 1) + fibo(num - 2)
    }
   
}
