// Check if the AC is on or not.
// current temperature 25 C.
// If the temperature of the room is 20 C or less, set the temperature to 27 C (heat mode) in the AC, if the temperature of the room is 30 C or more, turn on the cooling mode on 22 C in the AC. if AC is not working service AC.If room temperature is between 20 and 30 turn on fan.



fn ac_status() -> bool {
    return false;
}
fn ac_service() -> bool {
    return true;
}
fn main(){
    let room_temperature = 35.00;
    let mut ac_on = ac_status();
    let mut ac_mode = ["Heat", "Cool"];
    let mut cur_mode = "";
    let mut ac_temperature = 25.00;

    if room_temperature <= 20.00 {
        ac_on = true;
        // ac_mode[0] = "Heat";
        cur_mode = ac_mode[0];
        ac_temperature = 27.00;
        // println!("The Ac Temperature is set to {}", ac_temperature);
    }
    else if room_temperature >= 30.00 {
        ac_on = true;
        // ac_mode[1] = "Cool";
        cur_mode = ac_mode[1];
        ac_temperature = 22.00;
        // println!("The Ac Temperature is set to {}", ac_temperature);
    }

    if ac_on {
        println!("Ac is ON in {:?} mode, temperature set to {}", cur_mode,ac_temperature);
    }
    else {
        println!("Ac is off...")
    }

    // if room_temperature != ac_temperature {
    //     println!("Ac Requires service");
    //     ac_service();
    // }
    // else {
    //     println!("AC Working Fine");
    // }
    
    if cur_mode == "Heat" && room_temperature == ac_temperature {
        println!("The Ac is Working fine...");
    }
    else if cur_mode == "Cool" && room_temperature == ac_temperature {
        println!("The Ac is Working fine...")
    }
    else {
            println!("Ac requires service!!!");
            ac_service();
    }
}