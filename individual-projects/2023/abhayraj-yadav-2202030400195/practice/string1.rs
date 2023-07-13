struct Item{
    name: String,
    bag: i32
}
 fn print_name(name:&str){
    println!("Name:{:?}",name);
 }
fn main(){
    let items = vec![
        Item{
            name:String::from("Fruit"),
            bag:3
        },
        Item{
            name:String::from("Vegetable"),
            bag:5
        },
        Item{
            name:String::from("Cereals"),
            bag:7
        },
        Item{
            name:String::from("Pulses"),
            bag:10
        }
    ];

    for item in items{
        print_name(&item.name);
        println!("Bag:{:?}",item.bag);
    }
}