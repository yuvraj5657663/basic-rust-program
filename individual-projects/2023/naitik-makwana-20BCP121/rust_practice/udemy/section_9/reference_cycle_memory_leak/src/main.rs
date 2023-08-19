use std::rc::{Rc,Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    next : Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}",self );
    }
}

fn main() {
    let x1 = Rc::new(RefCell::new(Node {next:None}));

    println!("count of x1 : strong : {:?}, weak : {:?}", Rc::strong_count(&x1), Rc::weak_count(&x1));
    
    let x2 = Rc::new(RefCell::new(Node {next : Some(Rc::downgrade(&x1))})); //using downgrade ownership is not shared with x2 but x2 has weak reference to x1

    println!("after creating x2,count of x1 : strong : {:?}, weak : {:?}", Rc::strong_count(&x1), Rc::weak_count(&x1));
    println!("count of x2: strong : {:?}, weak : {:?}", Rc::strong_count(&x2), Rc::weak_count(&x1) );

    let x3 = Rc::new(RefCell::new(Node {next : Some(Rc::downgrade(&x2))}));
    println!("after creating x3,count of x2 : strong : {:?} , weak :{:?}", Rc::strong_count(&x2), Rc::weak_count(&x2));
    println!("count of x1 : strong : {:?} : weak :{:?}", Rc::strong_count(&x1), Rc::weak_count(&x1) );

    (*x1).borrow_mut().next = Some(Rc::downgrade(&x3));
    println!("after x1 referencing x3,count of x3 : {:?}", Rc::strong_count(&x3));

    //above code will make a cycle where a->b->c->a

    println!("Ref count of x1 : strong : {:?} , weak :{:?}", Rc::strong_count(&x1), Rc::weak_count(&x1));
    println!("Ref count of x2 : strong : {:?} , weak :{:?}", Rc::strong_count(&x2), Rc::weak_count(&x2));
    println!("Ref count of x3 : strong : {:?} , weak :{:?}", Rc::strong_count(&x3), Rc::weak_count(&x3));

    println!("x1 : {:?}", x1); // incase of strong counter it will create error
}
