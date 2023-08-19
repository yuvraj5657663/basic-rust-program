fn main(){
    // COMPOUND DATA TYPES :
                    //    1--TUPLES------------ 
    // TUPLES ARE THE TYPES THAT CAN COLLECT VALUES OF MORE THEN ONE TYPE , HAVE FIXED LENGTH SO ONCE DECLARED CANNOT GROW OR SHRINK

    let name_age = ("kashita", 18);
    println!(" name={ } and age ={ } ",name_age.0,name_age.1);

    // OR WE CAN SIMPLY , PRINT USING NAME 
    println!("{:?}",name_age);

    // ASSIGNING VALUES TO THE VARIABLES FROM TUPLES 
   
    // THIS CONCEPT IS CALLED DESTRUCTURING
    // HERE WE CAN PRINT THE VALUES WE WANT OUT OF TUPLES 
    // AS FOR INSTANCE I HAVE PRINTED NAME ONLY AS BELOW 
let (name , age )= name_age;               
 println!("{}", name);                               
 
 let name=name_age.0;
 println!("{ }", name_age.0);

// NESTED TUPLES :

let nested_tuple = ("hello", 2005,1.2,(4,8));
// HERE THERE IS A TUPLE IN A TUPLE 
// SO HERE NOW TO DECOUPLE THE FOURTH ELEMENT OOF THE TUPLE THAT IS FIRST ELEMENT OF THE NESTED TUPLE WE WILL USE INDEXES 3 ANS 0 
// 3 IS FOR OUTER TUPLE CAUSE THE INDEXES STARTS FROM ZERO IN RUST
// 0 IS FOR NESTED (INNER) TUPLE AS IT THE FIRST ELEMENT OD THAT TUPLE 
let element= nested_tuple.3.0;

// to print first element of the nested tuple 
println!("{}",nested_tuple.3.0);

// to print the second element of the nested tuple 
println!("{}",nested_tuple.3.1);

// declaring empty tuple !!
let empty_tuple =  ();









}