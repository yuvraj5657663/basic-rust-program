use std::cell::RefCell;

fn main() {
    /* 
    let mut x = 10;
    let x1 = &x;
    let x2 = &x;
    let x3 = &mut x; // this line will occure an error ar the compile time because of the ownership rule that ..immutable borrow and mutable borrow can not co-exixts.
    */


    /*
    let mut x = RefCell::new(10);
    let x1 = x.borrow();
    let x2 = x.borrow();

    drop(x1);
    drop(x2);
    let x3 = x.borrow_mut();

    println!("{:?}", x);
    //this code will occure an error but it will arise at run time
    //advantage  : certain memory safe operations are allowed which is not allowed at run time
    drop(x3);
    println!("{:?}", x);
    //original varible cannot be access while variable is borrowed 
    */

    let x = RefCell::new(20);
    let mut x1 = x.borrow_mut(); // here with Refcell, A mutable reference of an immutable RefCell is allowed, which means we can change the value of immutable RefCell.
    *x1 = 30; 
    drop(x1);

    // can directly change the value of immutable RefCell using borrow_mut function
    *x.borrow_mut() = 50;
    println!("{:?}",x );

}
