enum Colour{
    red,
    black,
    white
}
struct Car{
    colour:Colour,
    rate:f64,
}

fn find_car(car:Car){
    match car.colour{
        Colour::black => println!("BLACK CAR"),
        Colour::red => println!("RED CAR"),
        Colour::white => println!("WHITE CAR")
    }
    println!("Rate:{:?}",car.rate);
}

fn main(){
    let red = Car{
        colour:Colour::red,
        rate:580000.00,
    };
    find_car(red);
    let black = Car{
        colour:Colour::black,
        rate:570000.00,
    };
    find_car(black);
    let white = Car{
        colour:Colour::white,
        rate:560000.00,
    };
    find_car(white);
    
}
