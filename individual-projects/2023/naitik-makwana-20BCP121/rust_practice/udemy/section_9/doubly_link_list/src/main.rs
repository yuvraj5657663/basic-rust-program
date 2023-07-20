//doubly link list -> list where each node has information about it previous and next nodes.
//should have explicit information abot head and tail
//2 pointer pointing to each node

use std::rc::Rc;
use std::cell::RefCell;

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

struct List<T> {
    head: Pointer<T>,
    tail : Pointer<T>,
}

struct Node<T> {
    element : T,
    prev : Pointer<T>,
    next : Pointer<T>,
}

impl <T: std::fmt::Display> Node<T> {
    fn new(element:T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element: element,
            prev : None,
            next : None,
        }))
    }
}

impl <T: std::fmt::Display> List<T> {
    fn new() -> Self {
        List {
            head : None,
            tail : None,
        }
    }

    fn push_front(&mut self, element:T) {
        let new_head = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            },

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_end(&mut self, element:T) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            },

            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove_front(&mut self) {
        if self.head.is_none() {
            println!("This is empty list");
        }else {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take(){
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail.take();
                        println!("List is empty afetr removal");
                        None
                    }
                }
            });
        }
    }

    fn remove_end(&mut self) {
        if self.tail.is_none() {
            println!("This is empty list");
        }else {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take(){
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.head.take();
                        println!("List is empty afetr removal");
                        None
                    }
                }
            });
        }
    }

    fn print_list(&self) {
        if self.head.is_none() {
            println!("[]");
            return;
        }else {
            let mut traverse = self.head.clone();
            while !traverse.is_none() {
                println!("{}", traverse.as_ref().unwrap().borrow().element);
                traverse = traverse.unwrap().borrow().next.clone();
            }
            println!();
        }
    }
}

fn main() {
    let mut list_1 : List<i32>= List::new();

    list_1.push_front(1);
    list_1.push_end(2);
    list_1.push_end(3);
    list_1.push_end(4);
    list_1.push_front(5);
    list_1.remove_front();
    list_1.remove_end();
    list_1.print_list();

}
