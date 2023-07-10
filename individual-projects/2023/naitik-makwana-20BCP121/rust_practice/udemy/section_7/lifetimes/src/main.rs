// fn main() {
//     let some_num:i32 = 10;
//     let additional_num:&i32 = fun(&some_num);
//     println!("adress of some num is : {}",additional_num );
// }

// fn fun(i:&i32) -> &i32 {
//     &i
// }


/******************************************************************/


// fn main(){
//     let str1:&str = "Hello";
//     let x;
//     {
//         let str2 = String::from(" World");
//         v = fun1 (str1,str2.as_str());
//     }
//     println!("{}", v);
// }

// // lifetime specifier generics are needed when we want ref as a returning value of function

// fn fun1<'a,'b>(s1:&'a str, s2:&'b str) -> &'a str {
//     s1
// }


/********************************************************************/

// fn main() {
//     let x1:i32 = 5;
//     let x2:i32 = 6;

//     let result:&i32 = greater(&x1,&x2);
//     println!("{}",result );
// }

// fn greater<'a>(i:&'a i32, j: &'a i32) -> &'a i32 {
//     if i>j {
//         i
//     }else{
//         j
//     }
// }

/********************************************************************/


// fn main() {
//     let x1:i32 = 5;
//     let x2:i32 = 6;

//     let result:&i32 = greater(&x1,&x2);
//     println!("{}",result );
// }

// fn greater<'a, 'b>(i:&'a i32, j: &'b i32) -> &'a i32 {
//     if i>j {
//         i
//     }else{
//         j
//     }
// }


/********************************************************************/


// fn main() {
//     let x1:i32 = 5;
//     {
//         let x2:i32 = 6;

//         let result:&i32 = greater(&x1,&x2);
//         println!("{}",result );
//     }

// }

// fn greater<'a, 'b>(i:&'a i32, j: &'b i32) -> &'a i32 {
//     if i>j {
//         i
//     }else{
//         j
//     }
// }

/********************************************************************/

// struct Student<'a> {
//     name:&'a str,
//     roll:u32,

// }

// fn main() {
//     let s1_name = "Naitik";
//     let mut naitik:Student = Student {
//         name : &s1_name,
//         roll:21
//     };

//     {
//         let s_name = String::from("Makawana");
//         naitik.name = &s_name;
//         //will work here
//         println!("The name is : {} and age is : {}", naitik.name, naitik.roll );
//     }
//     //will not work here
//     //println!("The name is : {} and age is : {}", naitik.name, naitik.roll );
// }

/****************************************************************************/

fn main(){
    let my_vec = vec![1,2,3,4,5];
    let return_vec = use_vec(&my_vec, &my_vec);

    println!("{:?}",return_vec );

}

fn use_vec<'a>(vec1:&'a [i32], vec2:&'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    }else {
        vec2
    }
}