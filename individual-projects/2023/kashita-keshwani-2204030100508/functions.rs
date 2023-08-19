// functions are the  code fragment or segments that are used to perform any specific task !!
// fuctions are always written outside the main function !!
// to execute any of the function it is necessary to call them into the main function  !!
// functions are declared using the keyword fn 
//  { }=these curly brackets indicates the starting and ending of the function 
use std::io;
fn main() {
    basic_fn();

function_with_inputs("kashita", 40_000);
function_with_inputs("asha", 50_000);

 let full_name = "keshwani_kashita";
 let salary_info = 50_000;
function_with_inputs(full_name , salary_info);

 let answer= function_with_inputs_and_outputs(10,15);
 println!(" the answer is = {}",answer);

 
let (multiplication,addition,substraction) = function_with_inputs_and_multiple_outputs(10,15);
 println!("multiplication = {} , addition = {} , substraction = {}", multiplication,addition,substraction);
 }


fn basic_fn()
  {
println!("Basic function with no inputs and outputs");

                                                //    ----------------  //function body
  }      


  
  //   ----FUNCTIONS WITH INPUTS------------ 

  fn function_with_inputs(name: &str , salary : i32){
println!("my name = {} and salary = {} ",name, salary);
  }

  //   ----FUNCTIONS WITH INPUTS AS WELL AS OUTPUTS------------
//   outputs from the functions are specified by a dash and greater sign| -> | then data type of the output
  fn function_with_inputs_and_outputs(num1 :i32 , num2 : i32) -> i32 {
    num1 * num2
  } 

  fn function_with_inputs_and_multiple_outputs(num1:i32,num2:i32,) -> (i32,i32,i32) {
(num1*num2 , num1+ num2 , num2-num1)
  }

  // ---------------------INPUTS FROM USER--------------------------

  // to get input from user we need to include library that is = std::io ;
  // (look at the beginning of code)

  // io : : stdin()         // io::stdin is a function provided by the std::io module. it allows us to read the user input from the console !! (standard input stream)
    // .read_line()        //  It is used to read a line of input from a given input source, such as a FILE or the STANDARD INPUT STREAM.
    // It takes a mutable reference to a STRING  as the BUF parameter and returns a Result indicating the success or failure of the read operation. 

    // , the .read_line() method is called on io::stdin() to read a line of input from the STANDARD INPUT STREAM !!
    // .read_line() with sources other than the standard input stream, such as FILES, may require working with different types that implement the Read trait, like std::fs::File. !!

    //  example of trim function 
    // trim function removes all the white spaces from the string at the beginning and ending of the string statement  

    fn main() {
    let string_with_whitespace = "    Hello,     Rust!    ";
    
    println!("Original string: '{}'", string_with_whitespace);
    
    let trimmed = string_with_whitespace.trim();
    println!("Trimmed string: '{}'", trimmed);
}

