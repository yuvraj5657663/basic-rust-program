fn main() {
    let mut n = 0; // n is a over of match
    let total_over = 20;
    while n <= total_over {
        if rain == true { // random over this argument is true
            println!("Stop rain is start..")
            break;
        }
        else{
            println!("Play Game...")
        }
        n+=1;
    }

    rain = false;
    while n <= total_over {
        println!("Continue after rain...")
    }
}








overs = 20;
current_score = 0;
balls = 0;
wickets = 0;

fn game(balls, overs, wickets) {

    ball_result = ["dot ball","single","double","boundary","wicket", "rain"]
    while (balls < overs * 6 and wickets < 10){

        balls += 1
        if random.choice(ball_result) == "dot ball"{

            print("dot ball");
        }
        else if random.choice(ball_result) == "single"{

            current_score =+1;   
            print("Single taken. Current_score:", current_score);
        }
        else if random.choice(ball_result) == "double"{

            current_score += 2;
            print("Double taken. Current_score:", current_score);
        }
        else if random.choice(ball_result)== "wicket"{

            wickets += 1;
            print("Wicket taken. wickets:", wickets);
        }
        else if random.choice(ball_result) =="rain"{

            print("Rain is start. Stop the game!");
            new_game();
            break;
            // continue;
        }
    }
}

fn main() {
    game(balls, overs, wickets);
}