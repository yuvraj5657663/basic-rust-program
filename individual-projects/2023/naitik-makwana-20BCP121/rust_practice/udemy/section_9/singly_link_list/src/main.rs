// singly linked list is uni-direction.
// it can traverse only in one way
// each node contains data and pointer to the next node of the list
// first node is known as head and last node is known as tail
type pointer<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Linkedlist <T: std::fmt::Debug + std::marker::Copy>{
    head:pointer<T>,
}

#[derive(Debug)]
struct Node <T: std::fmt::Debug + std::marker::Copy> {
    element : T,
    next : pointer<T>,
}

impl  <T: std::fmt::Debug + std::marker::Copy> Linkedlist<T> {
    fn create_empty() -> Linkedlist<T> {
        Linkedlist {
            head : None
        }
    }

    // fn add(&mut self, element:i32) {
    //     match self.head {
    //         None => {self.head = Some(Box::new(Node{element:element, next :None}))},
        

    //         Some(prev_head) => {
    //             let new_head =  Some(Box::new(Node {element: element, next: Some(prev_head)}));
    //             self.head = new_head       
    //         }
    //     }
    // }
    fn add(&mut self, element:T) {
        let prev_head = self.head.take();

        let new_head = Box::new(Node {
            element: element,
            next:prev_head
        });
        self.head = Some(new_head);
    } 

    fn remove(&mut self) -> Option<T> {
        let prev_head = self.head.take();
        match prev_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            },
            None => None
        }
    }
    
    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(x) => Some(x.element),
            None => None
        }
    }

    fn print_ll(&self) {
        let mut list_traverse = &self.head;
        while true {
            match list_traverse{
                Some(Node) => {
                    println!("{:?}",Node.element);
                    list_traverse = &Node.next
                },
                None => break
            }
            
        }
    }

}


fn main() {

    /*
    let list1 = Node{
        element:1,
        next : None
    };

    let list2 = Node {
        element :1,
        next : Some(Box::new(Node {
            element :3 , 
            next : Some(Box::new(Node {
                element : 4,
                next : None
            }))}))
    };

    let ll1 = Linkedlist {
        head : Some(Box::new(Node {element:1, next:None}))
    };

    let ll2 = Linkedlist {
        head : Some(Box::new(Node {element:1, next: Some(Box::new(Node {element :2 , next:Some(Box::new(Node {element : 3,next : None}))}))}))
    };

    let ll3 = Linkedlist {
        head:None
    };

    println!("linkedlist : {:?}",ll2.head.unwrap());
    */
    let mut list_1 = Linkedlist::create_empty();
    list_1.add(1);
    list_1.add(2);
    list_1.add(3);
    list_1.add(4);
    println!("{:?}", list_1);
    list_1.remove();
    println!("{:?}", list_1);
    println!("{:?}", list_1.peek());

    list_1.print_ll();
}
