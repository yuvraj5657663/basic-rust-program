// 1
// fn main (){
//   let x=3;
//   let y=4;

  
//   let addition_3 =  add_3(x);
//   println!("{}",addition_3);
  
//   let addition_5 = add_5(y);
//   println!("{}",addition_5);  
  
//   let multiplication=times(x,y);
//   println!(
//              "The result of x+3 times y+5 is {}",
//             times(add_3(x), add_5(y)) );
// }

// fn add_3(x:i32) -> i32{
//   x+3
// }

// fn add_5(y:i32) -> i32{
//  y+5
// }

// fn times(x:i32,y:i32) -> i32{
//   x*y
// }
// ---------------------------------------------------------------------------------------------

// 2
// fn main() {

//     // let x = triple(double(5));

//     // let y = triple(x);
//     println!("Answer {}",triple(triple(double(5))));

// }fn double(x: i32) -> i32 {

//     x*2

// }

// fn triple(x: i32) -> i32 {

//     x*3

// }

// ---------------------------------------------------------------------------------------------

// 3
// fn main() {
//     println!(
//         "The distance of the number the point from the origin is {}",
//         print_distance((5.0, 4.0))
//     );
// }

// fn print_distance(point: (f32, f32)) -> f32 {
//     let (x, y) = point;
//     (x.powf(2.0) + y.powf(2.0)).sqrt()
// }
// ----------------------------------------------------------------------------------------------
// quadruple = it means multiplying by four !!
// 4
// fn main() {
//     println!(
//         "For 1: the expected value is 4 while the output is {}",
//         quadruple(1)
//     );
//     println!(
//         "For 2: the expected value is 8 while the output is {}",
//         quadruple(2)
//     );
//     println!(
//         "For 3: the expected value is 12 while the output is {}",
//         quadruple(3)
//     );
//     println!(
//         "For 4: the expected value is 16 while the output is {}",
//         quadruple(4)
//     );
// }
// fn double(x: i32) -> i32 {
//     x * 2
// }

// fn quadruple(x: i32) -> i32 {
//     double(double(x))
// }