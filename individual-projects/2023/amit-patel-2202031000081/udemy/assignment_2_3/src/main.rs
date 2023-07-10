fn main() {
  /*The distance between two points p1 with coordinates (x1, y1) and p2 with coordinates (x2,y2) is computed using the formula

sqrt ((x1 - x2)^2 + (y1-y2)^2 )   where ^ = is used to indicate the exponent. Write a program which will first declare two points p1 and p2 using tuples and will Initialize the tuple p1 from the values of (4.0, 3.0) and the tuple p2 from the values of (5.0, 4.5). Next, write a statement for determining their distance and then display the result to the terminal terminal.  */


 let p1 = (5.0,6.5);
 let p2 = (8.5,3.0);

 println!("distense btw two point p1 and p2 is {}.",((p1.0 as f32-p2.0 as f32).powf(2.0)+(p1.1 as f32-p2.1 as f32).powf(2.0)).sqrt());

}
