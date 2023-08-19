// REFERENCES  (&)

// reference can be given by using & before the variable name !!
fn main()
{
    let mut x = 10 ;
    let xr= & mut x;
    // the mutable reference of any value can be taken once only !!
    // else if needed it can be taken out of that scope !!
    
    // immutable reference can be taken multiple times !!
    *xr += 5 ;                          //changed the value after reference !!
    
    println!("x is {}", xr);
    
    // in case of non primitive data type like string type ,values are not copied but moved because such data is stored in "HEAP" frame so either reference can be given or .clone() function can be used to borrow the value !!

//   in case of primitive data types data in stored in "STACK" frame !!
//  here values are copied !!

// string literal are fixed in size and are stored in binary ("str")!! if we want string that is dynamic in size then we have to use string type ("string : : from("--" )") !! 

// mutable and im mutable references can not co-exist within the same scope !!
}  
