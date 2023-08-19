fn main() {
    println!("START CALCUTING");
    loop{
    println!("Enter number acording the options: /n 1.additon /n 2.subtraction /n 3.multiplication /n 4.division /n 5.modulus /n 6.squre_root /n 7.exit");
    let  mut option:i32 =input();
    if option <=5 && option>0{
        println!("Enter the first number:");
        let oprand_1=input();
        println!("Enter second number:");
        let oprand_2=input();
        match option{
            1=> println!("{} + {} = {}",oprand_1,oprand_2,addition(oprand_1,oprand_2)),
            2=> println!("{} - {} = {}",oprand_1,oprand_2,subtraction(oprand_1,oprand_2)),
            3=> println!("{} * {} = {}",oprand_1,oprand_2,multiply(oprand_1,oprand_2)),
            4=> println!("{} / {} = {}",oprand_1,oprand_2,division(oprand_1,oprand_2)),
            5=> println!("{} % {} = {}",oprand_1,oprand_2,modulus(oprand_1,oprand_2)),
            _=> println!("chose another option!!"),

        };
    }
    else if option == 6{
        println!("Enter the number :");
        let option_3=input();
        println!("Squre root of number is {}.",squre_root(option_3 as f32));
    }
    else if option == 7{
        println!("wish you get setisfied answer!!"); 
        break;
    }
    else{
        println!("invalid input", );
    }
    println!("Wants to try again?  1.yes or 2.no.");
    let last_conformaion=input();
    if last_conformaion==2{
        println!("Thank you!!!");
        break;
    }
    

    } 
}
fn input() -> i32{
    let  mut option=String::new();
    std::io::stdin().read_line(&mut option).expect("invalid input!");
    let option=option.trim().parse::<i32>().expect("invalid input");
    option 
}
fn addition(o1:i32,o2:i32) ->i32{
    o1+o2
}
fn subtraction(o1:i32,o2:i32) ->i32{
    o1-o2
}
fn multiply(o1:i32,o2:i32) ->i32{
    o1*o2
}
fn division(o1:i32,o2:i32) ->f32{
    (o1 as f32)/(o2 as f32)
}
fn modulus(o1:i32,o2:i32) ->i32{
    o1%o2
}
fn squre_root(o1:f32) -> f32{
    o1.sqrt()

}