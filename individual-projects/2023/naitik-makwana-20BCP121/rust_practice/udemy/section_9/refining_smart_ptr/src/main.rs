//code below used to prevent allocation of Box for the null 

/*
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

fn main() {
    
    let list = List::Cons(1,Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3,None)))))));

    println!("{:?}",list );
}
*/

// custom defined smart pointers
use std::ops::Deref;

struct MySmartPtr {
    val:i32
}

impl MySmartPtr {
    fn new(x:i32) -> Self {
        Self{
            val : x
        }
    }
}

impl Deref for MySmartPtr {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.val
    }
}

impl Drop for MySmartPtr {
    fn drop (&mut self) {
        println!("dropping structure");
    }
}

fn main() {

    let a = 1;
    let b = &a;

    let p1 = MySmartPtr::new(a);

    let p2 = MySmartPtr::new(*b);

    println!("{}", a == *p1);
    println!("{}", *b == p2.val);
    

    //drop function will automatically by program to release memory stored by structure
}  