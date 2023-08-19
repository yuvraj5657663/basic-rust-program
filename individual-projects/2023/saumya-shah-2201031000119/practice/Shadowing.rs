fn main(){
  let x = 5;
  let x = x + 1;
  {
    // Value inside this scope is limited to this scope
    let x = x + 5;
    println!("Inner-scope value of x : {x}");
  }
  println!("Outer-scope value of x : {x}");
}