fn main(){
  another_function(5);
  print_labeled_measurement(5, 'h');
}

fn another_function(x: i32){
    println!("Value of x is : {x}");    
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurment is: {value}{unit_label}");    
}