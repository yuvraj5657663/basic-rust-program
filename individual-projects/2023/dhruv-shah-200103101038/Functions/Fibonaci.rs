fn fibonacci(n){
    let mut a = 0;
    let mut b = 1;
    for i in n {
        let c = a + b;
        a = b;
        b = c;
        println!("{}", c);
    }
    println!("fibonaci series up to n:{}", n);
 }

 fn main(){
    fibonacci(20);
 }


//  recursion method

fn printfibonaci(i){
    if n == 0 {
        return 0;
    }
    else if n == 1 || n == 2 {
        return 1;
    }
    else {
        return printfibonaci(n-1) + printfibonaci(n-2);
    }
}
 
fn main(){
    for i in 0..4{
        println!("{}",printfibonaci(i));
    }

}


// f(0) = 0
// f(1) = 1
// f(2) = 1

// f(3)
// f(2) + f(1) = 2

// f(4)
// f(3)             + f(2)
  // f(2) + f(1)  +  1 
  // 1 + 1        + 1
  // 3

//  f(5)
//  f(4)           + f(3)
//  f(3)  + f(2)     +3
// 3 +1 +3 
// 8

// f(6)
// f(5)                  + f(4)
//   f(4)  + f(3)            +8
//   8 + 3 + 8 
// 19
   
