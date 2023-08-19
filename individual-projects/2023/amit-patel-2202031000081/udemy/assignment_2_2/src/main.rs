fn main() {
/*In this question, we will implement the same problem as disscussed in Question 1 but wtih the help of arrays. 

Declare two arrays of names p1 and p2 both having length of 2 and type f64. These two arrays will represent two points along with their coordinate values for the  x-axis and for y-axis. Write a simple program to display the absolute difference of the x-axis and the y-axis coordinates for the two points onto the terminal. */


let p1 :[f64;2] =[5.5,7.4];
let p2 :[f64;2] =[9.4,3.5];
println!("Diffrence btw X axis is {} and Y axis is {}. ",p1[0]-p2[0],p1[1]-p2[1]);
}
