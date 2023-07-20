use std::rc::{Rc,Weak};
use std::cell::{RefCell,Ref};
use std::borrow::Borrow;

#[derive(Debug)]
struct Node {
    val : i32,
    parent : RefCell<Weak<Node>>,
    child : RefCell<Vec<Rc<Node>>>
}

fn main() {
    let leaf = Rc::new(Node {
        val : 1,
        parent : RefCell::new(Weak::new()),
        child : RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        val : 5,
        parent : RefCell::new(Weak::new()),
        child : RefCell::new(vec![Rc::clone(&leaf)])
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); 

    println!("{:?}",branch );
}
