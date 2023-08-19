use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let x = Rc::new(RefCell::new(String::from("Hello")));

    // with rc we get multiple owners of data with immutable reference
    //if we wrap rc with refcell we can get multiple owner with mutable reference to data
    //ex. used in doubly linked list

    let y = Rc::clone(&x);

    *y.borrow_mut() =  String::from("Rust");

    println!("{:?}",x );
}
