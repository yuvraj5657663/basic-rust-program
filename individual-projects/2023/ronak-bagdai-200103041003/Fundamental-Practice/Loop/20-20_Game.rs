// fn main() {
//     let n = 20; // 20 overs
//     for i in 1..=n {
//         if rain {
//             break;
//         }
//        // code 1
//        // code 2
//     }

// }








// // Create a loop for 20 20 game. Stop the game if its raining

// let overs = 20;
// let current_score = 0;
// let balls = 0;
// let wickets = 0;
// let is_raining = true;

// fn main(){
//     let result = game(balls,overs, wickets);    
// }


// fn game(balls, overs, wickets){
//     while (balls < overs * 6  && wickets < 10 && !checkIfRaining()):
//             balls++;
//             ball_result = ["dot ball","single","double","boundary","wicket"]
//             //update the result based on the ball_result
//             if ball_result == "dot ball":
//                 print("dot ball")
//             else if ball_result == "single":
//                 current_score =+1   
//                 print("Single taken. Current_score:", current_score)
//             else if ball_result == "double":
//                 current_score += 2
//                 print("Double taken. Current_score:", current_score)
//             else if ball_result == "wicket":
//                 wickets += 1
//                 print("Wicket taken. wickets:", wickets)

//     return [current_score, wickets, balls, overs]
// }


fn main() {
    let mut raining = false;

    for overs in 1..=20 {
        println!("{} overs played", overs);

        if overs == 10 {
            raining = true;
            println!("It's raining, stopping play");
            break;
        }
    }

    if raining {
        println!("Resuming play after rain stops...");
        for overs in 11..=20 {
            println!("{} overs played", overs);
        }
    }
    println!("Game finished after 20 overs");
}
