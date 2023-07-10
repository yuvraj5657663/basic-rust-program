// topic : debug

/*
Fix the issue in the above code (see FIXME) so that it runs without error.
Try uncommenting the line that attempts to format the Structure struct (see TODO)
Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
 */
fn main() {
    //task 1
    println!("My name is {0}, {1} {0}", "Bond", "James");

    //task 2
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    //println!("This struct `{}` won't print...", Structure(3));

    println!("This struct `{:#?}`", Structure(3));

    //task 3
    let pi = 3.141592;
    println!("{:.3}", pi);
}
