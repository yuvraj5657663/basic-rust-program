// lift repair

let mut floor = 0;
let lift_find = 5;
fn main(){
        for lift in 0..14 {
        if lift_find == floor {
            println!("plz repair the lift & come back to ground floor");
            return current_floor(floor);
            break;
        }
        else {
            println!("find it the another floor");
            floor+=1;
        }
    }
    }
    

    fn current_floor(floor){
        println!("the lift is in {} floor", floor);
    }












    // lift repair

// fn main(){
//     let mut floor = 0;
//     let lift_find = 5;
//             for lift in 0..14 {
//             if lift_find == floor {
//                 println!("plz repair the lift & come back to ground floor");
//                 for lift in 0..14{
//                     if lift_find == floor{
//                       floor--;
//                         println!("you arrived ground floor")
//                     }
//                     else{
//                         println!("plz go to down")
//                     }
//                 }
//                 break;
//             }
//             else {
//                 println!("find it the another floor");
//                 floor+=1;
//             }
//         }
//         }

        






// function main(){
//     let floor = 0;
//     let lift_find = 5;
//     let ground_floor = 0;
//             for (i = lift_find; i<=14; i++) {
//             if (lift_find == floor) {
//                 console.log("plz repair the lift & come back to ground floor");
//                 for (i = lift_find; i>=0; i--) {
//                     console.log("go for ground floor")
//                 }
//                 break;
//             }
//             else {
//                 console.log("find it the another floor");
//                 floor++;
//             }
//         }
//     }
    
//     main();