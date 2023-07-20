use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Null
}

use List::{Cons,Null};

fn main() {

    /* 
    let a = Cons(1,Box::new(Cons(2,Box::new(Null))));
    let b = Cons(3,Box::new(a));
    let c = Cons(4, Box::new(a)); //rust compiler will give error for this line because ownership of a was transferred to b
    */

    let l1 = Rc::new(Cons(1,Rc::new(Cons(2, Rc::new(Null)))));
    println!("count of ref after creating l1 = {}", Rc::strong_count(&l1));
    let l2 = Rc::new(Cons(3, Rc::clone(&l1)));
    println!("count of ref after creating l2 = {}", Rc::strong_count(&l1));
    let l3 = Rc::new(Cons(4, Rc::clone(&l1)));
    println!("count of ref after creating l3 = {}", Rc::strong_count(&l1));
}

//cloning in rc doesnt make deep copy. It is just a reference. and data will be drop when there is no reference left.