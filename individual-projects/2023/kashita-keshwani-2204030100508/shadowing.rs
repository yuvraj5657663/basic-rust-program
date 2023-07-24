fn main()
{

    // SHADOWING VARIABLES  :
    
    // shadowing variables refers to concept os using , updating , or declaring a variable with the same name which has been previously used or declared in the program !!

    let s = 23;
    let s = 24;

    println!("value of s = {}",s);

    // so basically compiler sees the most recent value declared in program amnd executes it 
    // in this way the firstly declared variable being shadowed by the second !!
            //  -------------------------------------------


    // now will change the data type of the variable 

    let _sky = 100;
    let _sky = 'A';
    println!("value of _sky= {}", _sky);


    // if we change the value of a variable in a scope then it would be considered valid only within that scope !!

    // for instance
    let cloud = 50;
    {
        let cloud=100;
        println!("the value of cloud ={}",cloud);
    }
        println!("the value of cloud ={}",cloud);

        // now if we remove the let keyword from insideof the  scope variable then the value of the variable will be permanently changed !!

        // ----------------------------------------------------------------------------------


// CONST KEYWORD IS USED TO DECLARE A CONSTANT VARIABLE :

// for instance !
const  b : i32 = 25;     
println!("{}",b);
    
}