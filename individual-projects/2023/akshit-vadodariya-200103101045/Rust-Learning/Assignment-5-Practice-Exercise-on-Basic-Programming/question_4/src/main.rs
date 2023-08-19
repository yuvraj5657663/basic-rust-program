/* A palindrome is a word, verse, or sentence that reads the same backward or forward, such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not. The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.

 */

// use std::io;

fn is_palindrome(text: &str) -> bool {
    for i in 0..text.len() {
        let _char: Option<char> = text.chars().nth(i); // characters
        println!("char: {:?}", _char);
        let reverse = text.chars().nth(text.len() - i - 1); //reverse characters
        println!("char: {:?}", reverse);
        if reverse != _char {
            return false;
        }
    }
    return true;
}

fn main() {
    // println!("Enter string:");
    // let mut text = String::new();
    // io::stdin()
    //     .read_line(&mut text)
    //     .expect("Failed to read line");
    // let _palindrom = is_palindrome(text.as_str());

    let text = "Able was I ere I saw Elba";
    let _palindrom = is_palindrome(text);

    if _palindrom {
        println!("Yes, this string is palindrome")
    } else {
        println!("No, this string is not palindrome");
    }
}
