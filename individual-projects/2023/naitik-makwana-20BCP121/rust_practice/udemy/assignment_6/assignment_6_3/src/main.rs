// Section-5 // Assignment-6

/* Problem - 3
You are tasked with implementing a library management system using Rust. Your goal is to design a program that can handle books and magazines. To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input and displays its information. The function should output the item's ID, title, publication year, and type (book or magazine).
*/

struct Item {
    id : u32,
    title : String,
    year : u32,
    book_type : ItemType 
}

impl Item {
    fn display_item_info(&self) {
        println!("Book id : {}", self.id);
        println!("Title : {}", self.title);
        println!("Year : {}", self.year);
        println!("Type : {:?}", self.book_type);   
    }
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine
}

fn main(){

    let item1 = Item {
        id : 1,
        title : String::from("Rich Dad, Poor Dad"),
        year : 1997,
        book_type : ItemType::Book
    };

    let item2 = Item {
        id : 2,
        title : String::from("TIME"),
        year : 2023,
        book_type : ItemType::Magazine
    };

    item1.display_item_info();
    println!("***************");
    item2.display_item_info();
}