// Section-6 // Assignment-7

/* Problem - 4
Correct the code below so that it compiles
*/

struct Book<'a> {

    author: &'a str,

    title: &'a str,

}



fn main() {

    let name = String::from("Jill Smith");

    let title = String::from("Fish Flying");

    let book = Book {

        author: &name,

        title: &title,

    };



    println!("'{}' by '{}'", book.title, book.author);

}