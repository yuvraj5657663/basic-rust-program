// Section-6 // Assignment-7

/* Problem - 3
You are developing a text processing utility in Rust. Your task is to implement a function called find_longest_word() that takes a string slice as input and returns a reference to the longest word in that string slice. You will need to handle cases where there are multiple words with the same length, in which case the function should return the first occurrence.

Write the find_longest_word() function, ensuring that the lifetime parameter is properly handled to ensure the validity of the returned reference
*/

fn find_longest_word<'a>(s1:&'a str) -> &'a str {

    let vec1:Vec<&str> = s1.split_whitespace().collect();
    let mut long = "";
    let mut len = 0;

    for word in vec1.into_iter() {
        if word.len() > len {
            long = word;
            len = word.len()
        }

    }

    long
}

fn main(){

    let string1 = "Helloo this is a string";

    println!("The longest word in string is : {}", find_longest_word(&string1));
}