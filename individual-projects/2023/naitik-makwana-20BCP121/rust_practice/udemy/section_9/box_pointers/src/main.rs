// enum List {
//     Cons(i32,List),
//     Null
// }

#[derive(Debug)]
enum List {
    Cons(i32,Box<List>),
    Null
}

use List::{Cons,Null};
fn main() {

    //let list = Cons(1,Cons(2,Cons(3,Cons(4,(Cons(5,Null))))));
    //above line will give error because compiler dont know how much space Cons type will contain

    //Solution to this problem is
    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Cons(4,Box::new(Cons(5,Box::new(Null))))))))));
    println!("{:?}",list );

    //Cons list is a data structure, it contains a chained items inside the heap.
    //elements can be traverse using link or chain
    //similar to link list
}
