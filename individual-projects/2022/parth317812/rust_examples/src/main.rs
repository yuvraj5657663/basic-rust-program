fn main() {
    // mutability


    let mut x = 5;
    println!("{x}");

    x = x + 5; 
    println!("{x}");


    // scopes


    let x = "hello world";
    println!("first - {x}");

    {
        let x = "second time hello world";
        println!("second - {x}");
    }

    println!("third - {x}");


    // shadowing


    let x = 5;
    println!("initial - {x}");
    
    let x = 10;
    println!("shadowed - {x}");


    // if_else
    

    let x = 0;
    if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is not equal to t");
    }

    let y = 5;

    if y % 2 == 0 {
        println!("y is divisible by 2");
    } else if y % 3 == 0 {
        println!("y is divisible by 3");
    } else {
        println!("y is not divisible bt 2 or 3");
    }

    let t = if x == y {"x is equal to 5"} else { if x == 0 {"x is 0"} else {"x is not zero"} };
    println!("{t}");


    // loops
    

        // 1. type - loop

    let x = 655;
    let value = 'one: loop {
        if x >= 2 {
            break 'one x;
        }
        break 0;
    };
    if value != 0 {println!("your matching vaue is - {value}")} else {println!("no matching values are found!!!!!")}

        // 2. type - while

    let mut y = 5;
    while x >= 0 {
        println!("x is not negative {y}");
        y -= 1;
        if y < 0 {
            break;
        }
    }

        // 3. type - for 

    let a = [1,2,3,4,5,6,7,8,9];
    print!("through itrations: ");
    for i in a {
        print!("{i} ");
    }
    println!();
    print!("through range: ");
    for i in 0..9{
        print!("{} ", a[i]);
    }

    //patterns using for loops

    for i in 0..6 {                           
        for _j in i..6 {
          print!(" * " );
        }
        println!();
      }
    
      println!();
          // output
    
          // *  *  *  *  *  * 
          // *  *  *  *  *
          // *  *  *  *
          // *  *  *
          // *  *
          // *
    
          for i in 0..6 {                           
            for _j in 0..i+1 {
              print!(" * " );
            }
            println!();
          }
    
          // output
    
          // *
          // *  *
          // *  *  *
          // *  *  *  *
          // *  *  *  *  *
          // *  *  *  *  *  *
          println!();
          
          for i in 1..6 {
            print!("{}", "   ".repeat(5-i));
            print!("{}", " * ".repeat(i));
            println!();
          }
          
          println!();
    
          let mut i = 4;
    
          for j in 0..5 {
            for _ in 0..i {
              print!(" ");
            }
            i = i - 1;
            
            for _ in 0..j+1 {
              print!("* ");
            }
            println!();
          }
    
          // output
    
          //     *
          //    * *
          //   * * *
          //  * * * *
          // * * * * *

}


