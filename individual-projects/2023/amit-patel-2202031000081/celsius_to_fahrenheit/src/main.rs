fn main() {
    println!("Enter the temprature in the celsius:");
    let  mut temp = String::new();
    std::io::stdin().read_line(&mut temp).expect("faild to read the temprature");
    
    let temp:f32=temp
    .trim().parse()
    .expect("faild to read temp");
    
    
    let fahrenheit = (temp *( 9.0 / 5.0) )+ 32.0;
    println!("Temprature in fahrenheit is {}F.",fahrenheit );

}
