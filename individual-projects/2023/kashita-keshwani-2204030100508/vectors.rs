fn main() {
    // ---------------------VECTORS---------------------
    // VECTORS ARE THE COLLECTION OF SIMILAR ELEMENTS THAT CAN BE RESIZED  
    
                        // : vec<i32> 
    let mut kash                   = vec![1,2,3,4,5];
    println!("{}", kash[0]);
    println!("{:?}", kash);

    // like arrays we can also initialize vectors containing same values 
    // syntax
     let number_system = vec![0;10];
    //  this will print 10 zero values 
     println!("{:?}",number_system);

//   STRING VECTORS 
     let mut string_vectors = vec!["pizza","burgiiirr","sandwich"];
     println!("{:?}",string_vectors);

// initializing same string value vectors  !!
     let string_unknown_vectors =  vec!["unknown" ;5];
     println!("{:?}",string_unknown_vectors);

    //  updating vectors 
    // this will update the first element of the string_vectors 
    string_vectors[0]= "garlic_bread";

    // initializing same character value vectors  !!
    let mut char_vectors = vec!['a','b','c' ,'d','e'];

    // CREATING VECTOR SLICE (REFERENCES)
    let some_character = &&char_vectors[0..3];
    println!("{:?}",some_character);


    // ------------------FUNCTIONS ON VECTORS-------------------

//    to get the length of vector                                                           =            len();
//    to get the index number                                                              =            get();
//    to push/add any value                                                                 =             push();
//    to remove any value                                                                    =           remove();
// to see if the existing vector contains the particular value or not                         =          contains();  

// examples 
println!("{}", char_vectors.len());                    //length

println!("{:?}",char_vectors.get(2));                //check index

char_vectors.remove(1);                               //remove value
println!("{:?}", char_vectors);

char_vectors.push('f');                                   //add value 
println!("{:?}", char_vectors);

println!("{}", char_vectors.contains(&'a'));      //check if value exists or not 


}