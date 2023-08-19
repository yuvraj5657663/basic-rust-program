// Section-5 // Assignment-6

/* Problem - 1
Consider the program below. Modify it to eliminate the usage of all the let statements, both inside the main function and the other functions.

By removing the unnecessary let statements, the code becomes more concise and avoids unnecessary variable assignments.

Please note that the revised code is functionally equivalent to the original code, but it has been modified to eliminate the let statements for improved readability and simplicity.
*/

struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn increase_fruit(& mut self) {
        self.apples *= 2;
        self.bananas *= 3;
    }
    
    fn new_fruit() -> Self {
        Fruit {
            apples: 10,
            bananas: 5,
        } 
    }
     
    fn print_fruit(&self) { 
        println!("You have {} apples and {} bananas", self.apples, self.bananas);
    }
}

fn main() {
    let mut fruit_bag = Fruit::new_fruit();
    fruit_bag.increase_fruit();
    fruit_bag.print_fruit();
}