fn main(){
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    
    println!("value of x : {x}");
    println!("value of y : {y}");
    println!("value of z : {z}");

  let a: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = a.0;

    let six_point_four = a.1;

    let one = a.2;
    
    println!("First value of a : {five_hundred}");
    println!("Second value of a : {six_point_four}");
    println!("Third value of a : {one}");
}