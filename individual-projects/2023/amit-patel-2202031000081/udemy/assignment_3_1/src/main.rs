fn main() {
 //assignment 3.1

 let x = (5 + 3) * (6 + 4);
 let y = times(add_3(5),add_4(6));
 assert_eq!(x, y);
 println!("nice work!");
 

}
fn add_3(x:i32)-> i32 {
    x+3
}
fn add_4(x:i32) -> i32  {
x+4

}
fn times(x:i32,y:i32) -> i32  {
    x*y
}




/* Question 3.1
Modify the program below by adding the definition of the functions "add()" so that it compiles correctly



fn main() {

    let x = (5 + 3) * (6 + 4);

    let y = times(add_3(5), add_4(6));

    assert_eq!(x, y);

    println!("Good job!");

}
 */