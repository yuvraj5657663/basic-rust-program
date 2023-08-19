use array_tool::vec::*;
use crate_exp::*;

fn main() {
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![4,5,6,7,8];

    let intersect = v1.intersect(v2.clone());
    println!("{:?}",intersect );
    let uni = v1.union(v2.clone());
    println!("{:?}",uni );

    let x1 = 2;
    println!("The square is : {}",square(x1));
    let x2 = 3;
    println!("The cube is : {}",cube(x2));
    let x3 = 5;
    println!("The doublr is : {}", double(x3));
}
