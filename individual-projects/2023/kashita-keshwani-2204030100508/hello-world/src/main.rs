fn main() {
    println!("Hello, world!");

   //  let mut n =  0;

    //    INFINITE LOOP -------- 
    // loop{
    //     n+= 1;
    // 
    //     println!("the value of n= {}",n);
    // } 

      //   DEFINITE LOOP ---------- 
//      loop{
//         n+= 1;

//         if n == 7{               // |
//             continue;           // |this is used to skip any number 
//         }                             // |

//         if n>10{
//             break;
        
//     }
//         println!("the value of n= {}",n);
// }

//   WHILE LOOP------------ 
 let  mut n = 2;
      //  multiple of 2-- 
 while n <=20{
 if n % 2==0 {
    println!(" value of n = {}",n);
   }
    n +=1;
}

for i in  1..11{
   println!("the number is  {}",i);
}

let numbers = 20..40;

for i  in numbers {
   println!("the number is {}",i);
}
   // use of for loop to print vectors  -------

let food_items = vec!["sandwich" , "pav_bhaji" , "vadapav"];

for (index, a) in food_items.iter().enumerate() {
 println!(" {} here is {}",index,a);
} 
}

