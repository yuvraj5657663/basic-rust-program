fn main() {
     println!("Enter string :");
     let mut enter_string=String::new();
     std::io::stdin()
     .read_line(&mut enter_string)
     .expect("enter fail");


     let enter_string:String=enter_string
     .trim()
     .to_string();
     
  if check_palindrome(enter_string){
             println!("Entred string is palindrome");
  }
  else{
            println!("Entred string  not is palindrome");
  }


}
fn check_palindrome(n:String) -> bool{ 
    let charecters:Vec<char>=n.chars().collect();
    let len= charecters.len();
    for c in 0..len/2{
        if charecters[c] != charecters[len-1-c]{
            return false;
        }

    }
    true

}


/*
Palindrome is a word, verse, or sentence (such as "Able was I ere I saw Elba") or a number (such as 1881) that reads the same backward or forward. Write a function called palindrome which will check if a given string is a palindrome or not. The input to the function will be String and the output will be a bool value.  */