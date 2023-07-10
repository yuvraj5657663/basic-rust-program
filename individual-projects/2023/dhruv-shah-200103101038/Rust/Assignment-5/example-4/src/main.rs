// Palindrome is a word, verse, or sentence (such as "Able was I ere I saw Elba")
//  or a number (such as 1881) that reads the same backward or forward.
//  Write a function called palindrome which will check if a given string is a palindrome or not.
//  The input to the function will be String and the output will be a bool value.


fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if is_palindrome(&input) {
        println!("The string is a palindrome.");
    } else {
        println!("The string is not a palindrome.");
    }
}

fn is_palindrome(input: &str) -> bool {
    let reversed_string: String = input.chars().rev().collect();
    input == reversed_string
}
