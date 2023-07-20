// Section-6 // Assignment-7

/* Problem - 4
Fix the program below by adding suitable lifetime specifiers.
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {

        x

    } else {

        y

    }

}



fn main() {

    let string1 = String::from("abcd");

    let string2 = "vwxyz";



    let result = longest(string1.as_str(), string2);

    println!("The longest string is '{}'", result);

}
