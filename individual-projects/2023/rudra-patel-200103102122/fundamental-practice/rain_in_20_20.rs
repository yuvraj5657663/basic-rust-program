// 20 20 cricket match

fn main(){
    let mut rain = false;
     let mut overs = 0;
 
     loop {
         if rain == true {
             println!("the match is stopped due to rain");
         } 
         else if rain == false {
             println!("the match is started");
         }
         else if overs == 20 {
             println!("the first inning is completed");
         }
         else if overs == 40 {
             println!("the match is completed");
         }
         else {
         
         }
      }
    } 




  






// fn main() {
//     let mut n = 0; 
//     let total_over = 40;
//     while n <= total_over {
//         if rain == true { 
//             println!("Stop rain is start..")
//             break;
//         }
//         else{
//             println!("Play Game...")
//         }
//         n+=1;
//     }

//     rain = false;
//     while n <= total_over {
//         println!("Continue after rain...")
//     }
// }


