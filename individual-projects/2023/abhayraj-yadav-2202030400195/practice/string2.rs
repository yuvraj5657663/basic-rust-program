struct Data{
    name: String,
    age:i32,
    colour:String
}

fn print_name(name:&str){
    println!("Name={:?}",name);
}
fn print_colour(colour:&str){
    println!("Favourite colour:{:?}",colour);
}
fn main(){
    let data_list = vec![
        Data{
            name:String::from("Abhay"),
            age:08,
            colour:String::from("Black")
        },
        Data{
            name:String::from("Raj"),
            age:11,
            colour:String::from("Red")
        },
         Data{
            name:String::from("Ram"),
            age:09,
            colour:String::from("Orange")
        }
    ];

    for list in data_list{
        if list.age <= 10
        {
        print_name(&list.name);
        println!("Age:{:?}",list.age);
        print_colour(&list.colour);
    }
        }
        
}