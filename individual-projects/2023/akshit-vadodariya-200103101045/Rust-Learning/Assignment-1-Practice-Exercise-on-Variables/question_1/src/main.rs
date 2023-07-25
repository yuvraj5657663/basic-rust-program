fn main() {
    // Consider the following program. Identify a few basic errors in the code.
    // my_age = 40;
    // my_son_age = 35

    // println!("My age is {} and my son age is {}", my_age, my_son_age);

    // 1. in above code variable not define using "let" keyword
    // 2. the end of line_no : 3 semicolon (;) missing 


    //  Right Code :-
    let my_age = 40;
    let my_son_age = 35;

    println!("My age is {} and my son age is {}", my_age, my_son_age);
}
