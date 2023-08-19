// Section-9 // Assignment-8

/* Problem - 1
Complete the code below to create a linked list using Box smart pointers. Each Node struct has a data field of type i32 and a next field of type Option<Box<Node>>. Your task is to assign the next field of each node appropriately to create a linked list. The final linked list should start with node1, followed by node2, and finally node3. Print the linked list at the end.

The expected output should be:

Linked list: Node { data: 1, next: Some(Node { data: 2, next: Some(Node { data: 3, next: None }) }) }

*/


#[derive(Debug)]

struct Node {

    data: i32,

    next: Option<Box<Node>>,

}



fn main() {

    let mut node1 = Node {

        data: 1,

        next: None,

    };

    let mut node2 = Node {

        data: 2,

        next: None,

    };

    let mut node3 = Node {

        data: 3,

        next: None,

    };



    // Insert your code here which will connect the nodes using Box pointers. Two statements are enough here
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));


    println!("Linked list: {:?}", node1);

}