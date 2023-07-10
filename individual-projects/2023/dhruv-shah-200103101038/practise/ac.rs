// Check if the AC is on or not.
// current temperature 25 C.
// If the temperature of the room is 20 C or less, set the temperature to 27 C (heat mode) in the AC, 
// if the temperature of the room is 30 C or more, turn on the cooling mode on 22 C in the AC.
// if AC is not working service AC.
// If room temperature is between 20 and 30 turn on fan.



fn ac_temp(){
    return 25;
}

fn service(){
    if room_temp != 27 || room_temp != 22{
        println!("service requird")
    }
}


fn main(){

    let ac_on = true;
    let room_temp = 16;


    if ac_on {
        if room_temp <= 20{
            println!("AC is on, set the temperature to 27 C");
            service();
        }
        else if room_temp >= 30{
                println!("AC is on, turn on the cooling mode on 22 C");
                service();
        }
        else{
                println!("AC is on, turn on fan");
        }
    }
    else{
        service();
    }
}

