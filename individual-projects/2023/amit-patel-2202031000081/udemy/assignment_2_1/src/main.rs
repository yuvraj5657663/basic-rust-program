fn main() {
    /*Define two tuples calls p1 and p2 which will represents points and will have two values, one for the x-axis and one for y-axis. Write a program to display the absolute difference of the values of x-axis and the y-axis on the terminal. */


     let p1 = (4.6,2.7);
     let p2 = (6.4,7.2);
     println!("diffrence of X axis is {} and Y is {}.",p1.0 as f32 -p2.0 as f32,p1.1 as f32-p2.1 as f32);

}
