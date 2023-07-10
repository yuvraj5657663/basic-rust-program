// Check if the AC is on or not.
// current temperature 25 C.
// If the temperature of the room is 20 C or less, set the temperature to 27 C (heat mode) in the AC, if the temperature of the room is 30 C or more, turn on the cooling mode on 22 C in the AC. if AC is not working service AC.If room temperature is between 20 and 30 turn on fan.

// Steps
// 1. check AC power on or off -> create fn ACpower -> is false -> AC is off, Turn on the AC
//                                         |
//                                         V
//                                      is true.
//                                         |
//                                         V        
//                           take user input for room temp ->   if room temp isequl or below 20 -> currentTemp + 2 
//                                       |       |                                                       |    
//                                       |       |                                                       V
//                                       |       |                                           if temp not increz -> service AC or Compresore
//                                       |       | 
//                                       |       L->  if room temp isequl or above 30 ->  currentTemp - 3
//                                       |                                                     |
//                                       |                                                     V
//                                       |                                            if temp not decrez -> service AC or Compresore
//                                       |
//                                       L->  if room temp is between 20 and 30 -> Turn on the fan
// 
// 2. current temperature 25 C. -> create fn currentTemp() -> 25 


let increzTemp = ACcurrentTemp() + 2;
let decrezTemp = ACcurrentTemp() - 3;
let userTemp = user_input; //room temp
let mut roomcurrentTemp = 0;

fn ACpower() {
    return true;
}

fn ACcurrentTemp() {
    return 25;
}

fn service() {
    if userTemp != roomcurrentTemp || userTemp != roomcurrentTemp {
        println!("AC or compresore service Required!");
    }
}

fn main() {
    if ACpower(){
        if userTemp <= 20 {
            roomcurrentTemp= increzTemp;
            println!("Set AC temperature {} C  {}" , increzTemp);
            service();
        }
        else if userTemp >= 30 {
            roomcurrentTemp = decrezTemp;
            println!("Turn on AC cooling mode {} C {}" , decrezTemp);
            service();
        }
        else {
            println!("Room temperature is between 20 and 30 Turn on Fan");
            service();
        }
    }
    else {
        println!("AC is off, Turn on the AC");
    }
}