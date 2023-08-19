// Section-4 // Assignment-5

/* Problem - 4
    Palindrome is a word, verse, or sentence (such as "Able was I ere I saw Elba") or a number (such as 1881) that reads the same backward or forward. Write a function called palindrome which will check if a given string is a palindrome or not. The input to the function will be String and the output will be a bool value.
*/

fn main () {
    println!("Enter a string ");
    let mut s1 = String::new();
    std::io::stdin().read_line(&mut s1).expect("fail");

    let s1 = s1.trim().to_string();
    println!("Is palindrome? : {}",palindrome_checker(s1) );

}

fn palindrome_checker (s : String) -> bool {
    
    let vec1:Vec<_> = s.chars().collect();
    let len = vec1.len(); 
    let mut flag = true;

    for i in 0..=(len/2){
        if vec1[i] != vec1[len-i-1] {
            flag=false;
            break;
        }
    }

    flag
}