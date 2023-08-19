fn main()
{

    // mut is a keyword that allow us to make a variable mutable i.e we can change the value of that variable n number os times
    let mut  x=15;
    println!("value of x = {}",x);

    x=20;
    println!("{}",x);   


    // declaring more then one variable in same line :

    let (first_num , second_num) = (250,260.6);
    println!(" first {} second {}",first_num,second_num);

//    while assigning another variable and adding two numbers of different data types will throw an error bacause the complier bocomes confused so we can just convert any of the variable as given below (AND IT WILL CONVERT TEMPORARILY ONLY FOR THAT PARTICULAR OUTPUT NOT FOR WHOLE CODE )   

     let third_num = first_num+second_num as i32;
     println!("third = {}", third_num);

     let a = 69;
     println!("the value of a in octal is {:o}", a);
     println!("the value of a in hexadecimal is {:x}", a);
     println!("the value of a in binary is {:b}", a);
    

    // SCALAR DATATYPES
    // INTEGERS :
        //   - SIGNED :  (indicated by i)
                    //    I8,I16,i32,i64

            //   - UNSIGNED :  (indicated by u)
                   //    u8,u16,u32,u64   
                         

//     RANGE OF  THESE SIGNED DATA TYPES:

//   println!("the maximum num in i8 = { }", std :: i8 :: MAX);
//   println!("the minimum num in i8 = { }", std :: i8 :: MIN);

//   println!("the maximum num in i16 = { }", std :: i16:: MAX);
//   println!("the minimum num in i16 = { }", std :: i16 :: MIN);

//   println!("the miximum num in i32 = { }", std :: i32 :: MAX);
//   println!("the minimum num in i32 = { }", std :: i32 :: MIN);
 
//  println!("the maximum num in i64 ={ }", std :: i64 :: MAX);
//  println!("the minimum num in i64 ={ }", std :: i64 :: MIN);

    // RANGE OF  THESE UNSIGNED DATA TYPES :

//  println!("the maximum num in U8 ={ }", std :: u8 :: MAX);
//  println!("the minimum num in U8 ={ }", std :: u8 :: MIN);
 
//  println!("the maximum num in U16={ }", std :: u16 :: MAX);
//  println!("the minimum num in U16 ={ }", std :: u16 :: MIN);  

//  println!("the maximum num in U32 ={ }", std :: u32 :: MAX);
//  println!("the minimum num in U32 ={ }", std :: u32 :: MIN);   

//  println!("the maximum num in U64 ={ }", std :: u64 :: MAX);
//  println!("the minimum num in U64 ={ }", std :: u64 :: MIN);    


// FLOAT DATATYPE

        //    ----------------------------------------
                //    f32  ,f64
        //    ----------------------------------------


//  println!("the maximum num in f32 ={ }", std :: f32 :: MAX);
//  println!("the minimum num in f32 ={ }", std :: f32:: MIN);
        
//  println!("the maximum num in f64 ={ }", std :: f64 :: MAX);
//  println!("the minimum num in f64 ={ }", std :: f64 :: MIN);

// BOOLEAN DATATYPE

       //  --------------------------------------
        //  TRUE
        //  FALSE
    //  ----------------------------------------



// example :
// let not_equals : bool = 18  !=18;
// println!("the value of the condition is ={}", not_equals);


// CHARACTER DATATYPE
//    (defined as char)
//     (comes ender single quote'..')

// example :
// let c1 : char ='a';
// let c2 : char ='b';
// let c3 : char ='c';
// let c4 : char ='d';

// println!("the value of c1={} c2={} c3={} c4={}",c1,c2,c3,c4);
}