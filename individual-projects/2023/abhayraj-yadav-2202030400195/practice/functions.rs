fn first_name() {
    print!("abhay");
}
fn last_name() {
    println!("raj");
}
fn age_name(name:char,age:i32){
    println!("The name is {:?} and is {:?} years old.",name,age);
}

fn add(n1:i32,n2:i32) -> i32{
     n1+n2
}
fn display_add(result:i32){
    println!("{:?}",result);
}
fn sub(n3:i32,n4:i32) -> i32{
     n3-n4
}
fn display_sub(result1:i32){
    println!("{:?}",result1);
}

fn main() {
  
    first_name();
    last_name();
    age_name('a', 20);
  let result = add(50,40);
  let result1=sub(30,20);
  display_add(result);
  display_sub(result1);

}
