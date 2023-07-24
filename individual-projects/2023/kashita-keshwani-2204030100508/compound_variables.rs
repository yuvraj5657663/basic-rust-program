fn main()
{

     // COMPOUND DATA TYPES !!
               //     ----------------------STRING----------------------

     // declaring a string variable  (&str)
     let mut some_string = "fixed string character";

     // declaring an end string variable (capital S)
     let mut some_string = String::from("fixed string character");

    println!("this is \"{}\"", some_string);

     //  push function helps  to push(add)character to the end of the word of the statement 
     some_string.push('s');    
     println!("this is \"{}\"", some_string);

     //  push function helps  to remove character from  the end of the word of the statement 
     // also it doesn't have any input
     some_string.pop();    
     println!("this is \"{}\"", some_string);

     // in case we need to add (push) more than one character in  in the string then will use push_str 
     some_string.push_str(" that is");    
     println!("this is \"{}\"", some_string);

     // THERE ARE SOME BASIC FUNCTIONS ON STRING 
     // I.e.,   
     // is_empty(): { };
     // length(): { };
     // bytes(): { };
     // contains use(): { };
     // capacity(); { };
     println!(
          "is_empty() : { },
           length(): { }, 
           capacity(): { },
           contains use(): { }",
 some_string.is_empty(),
 some_string.len(),
 some_string.capacity(),
 some_string.contains("use")
     );

//   suppose we have a variable some_char with value 'M' and we want to convert it into the string variable 
     let some_char = 'M';
     // then will use the{ (dot).to_string } string fuction 
      let the_char = some_char.to_string();
     println!("the_char={ }",the_char);  

//   declaring a function and converting it to the string at the same time
     let my_name = "kashita_keshwani" .to_string();

// want to print more then one different string values together , here is the command called format! macro we need to use 
               let s_1="kashita" .to_string();
               let s_2="keshwani".to_string();
               let s_3= format!("{ } { }",s_1,s_2);
               println!("{ }", s_3);

// similarly to combine aur concatenate the values format command is used 
let concate=format!("{} {}", s_1,s_2);
println!("{ }",concate)
}