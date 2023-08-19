fn main() {
    let mut j = 0;
    let lift = 5;
     for _i in 0..14 {
        if lift == j {
            println!("Repair it and come back in lift.");
            while true {
                if j < 14 && j > 0  {
                    println!("Back to ground floor using lift {}", j);
                    j-=1;
                }
            }
            break;
        }
        else {
            println!("Go one floor up {}", j);
            j+=1;
        }
     }
}






fn main() {
    let mut j = 0;
    let lift = 5;
     for _i in 0..14 {
        if lift == j {
            println!("Repair it and come back in lift.");
            for _i in 0..14 {
                if _i < lift{
                    println!("Back to ground floor using lift {}", j);
                    j-=1;
                }
            }
            break;
        }
        else {
            println!("Go one floor up {}", j);
            j+=1;
        }
     }
}