// fn main(){
//     let limit = 20;
//     let mut a = 0;
//     let mut b = 1;

//     while b <= limit {
//         print!("{} ", b);
//         let c = a + b;
//         a = b;
//         b = c;
//     }
// }   

// fn main() {
//     let limit = 20;
//     let mut a = 0;
//     let mut b = 1;
//     let mut series = [0; 7]; // define an array of size 7
//     let mut i = 0;

//     while b <= limit {
//         series[i] = b;
//         i += 1;
//         let c = a + b;
//         a = b;
//         b = c;
//     }

//     println!("{}", series);
// }


fn printFibonacci(n:i32) ->i32 {
    if n == 0 {
        return 0;
    }
    else if n == 1 {
        return 1;
    }
    else {
        return printFibonacci(n-1) + printFibonacci(n-2);
    }
}
fn main() {
    for i in 0..5 {
        print!("{} ", printFibonacci(i));
    }
}

when i = 0, n=0, return 0 and print 0  // 0
when i = 1, n=1, return 1 and print 1 // 0 1
when i = 2, n=2, return printFibonacci(1) + printFibonacci(0) so again call function and return 1 + 0  print 1// 0 1 1
when i = 3, n=3, return printFibonacci(2) + printFibonacci(1) so again call function and n=2 so check else return printFibonacci(1) + printFibonacci(0) so again call function and return 1 + 0 print 2 // 0 1 1 2