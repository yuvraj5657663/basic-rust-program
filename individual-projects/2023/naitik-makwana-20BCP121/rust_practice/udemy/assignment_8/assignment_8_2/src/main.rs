// Section-9 // Assignment-8

/* Problem - 2
You are developing a messaging application where users can send messages to each other. As part of the application, you need to create a Message struct that represents a single message. The struct should have fields for the sender's name and the content of the message.

Currently, the messaging functionality is not implemented, but it will be added in future updates of the application. When a message is sent out to multiple users, all of them should have ownership of the message. This is required to ensure that the user are able to read the message and consume it successfully.

To achieve this shared ownership, you need to use the Rc (Reference Counted) smart pointer. Rc allows multiple users to have shared ownership of the messages. It keeps track of the number of references to the message and deallocates the message when the last reference goes out of scope.

Your task is to implement the following functions for the Message struct:

new(): Takes the sender's name and message content as parameters and returns a new Message instance wrapped in an Rc.

sender_name: Returns a reference to the sender's name.

message_content: Returns a reference to the message content.

Use the code below for complete

*/


use std::rc::Rc;

struct Message {

    sender_name: String,

    message_content: String,

}



impl Message {

    fn new(sender_name: String, message_content: String) -> Rc<Self> {

        Rc::new(Self{
            sender_name : sender_name,
            message_content : message_content,
        })

    }



    fn sender_name(&self) -> &str {

       self.sender_name.as_str()

    }



    fn message_content(&self) -> &str {

        self.message_content.as_str()

    }

}



fn main() {

    let message = Message::new("Alice".to_string(), "Hello, Bob!".to_string());

    println!("Sender: {}", message.sender_name());

    println!("Message: {}", message.message_content());
}

