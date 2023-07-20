use std::ops::Deref;

struct MySmartPtr<T : std::fmt::Debug> {
    val:T
}

impl<T: std::fmt::Debug> MySmartPtr<T> {
    fn new(x : T) -> MySmartPtr<T> {
        MySmartPtr{
            val : x
        }
    }
}

impl<T: std::fmt::Debug> Deref for MySmartPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.val
    }
}

impl<T: std::fmt::Debug> Drop for MySmartPtr<T> {
    fn drop (&mut self) {
        println!("dropping structure");
    }
}

fn some_fn(s : &str) {
    println!("received string : {}", s);
}

fn main() {
    let ptr1 = MySmartPtr::new("Naitik");
    some_fn(&ptr1); //here deref will be called for structure type then chained deref will be implementewd for string type and check if deref can be implemented for string type, which is element of structure. This method is known as deref coercion

    let vec1 = vec![1,2,3];

    for i in &*vec1 {
        println!("{}", i);
    }
}
