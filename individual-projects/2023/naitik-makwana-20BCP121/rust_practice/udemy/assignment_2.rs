// Section-2 // Assignment-2 

 fn main() {
    //task 1
    let (p1,p2) = ((5.4 as f64).abs(),(-5.5 as f64).abs());
    let (x1,x2) = ((3.2 as f64).abs(),(-1.5 as f64).abs());

    let output:(f64,f64) = (p1-x1,p2-x2);
    println!("{:?}", output);

    //task 2
    let arr1: [f64;2] = [(5.4 as f64).abs(),(-5.5 as f64).abs()];
    let arr2: [f64;2] = [(3.2 as f64).abs(),(-1.5 as f64).abs()];

    let output2: [f64;2] = [arr1[0]-arr2[0], arr1[1]-arr2[1]];
    println!("{:?}",output2);

    //task 3
    let (p,q) = ((5.4 as f64),(-5.5 as f64));
    let (x,y) = ((3.2 as f64),(-1.5 as f64));

    let dist:f64 = ((p-x).powf(2.0) + (q-y).powf(2.0)).sqrt();
    println!("{}",dist); 

    //task 4
    let a:i32 = -15;
    let b:i32 = 170;
    let my_name: &str = "Naitik";

    println!("My name is {}, and the answer is {}", my_name, a * b);
}

/* Problem - 1
Define two tuples calls p1 and p2 which will represents points and will have two values, one for the x-axis and one for y-axis. Write a program to display the absolute difference of the values of x-axis and the y-axis on the terminal.

Note: use the abs() function for determining the absolute value of the difference. The compiler may complain when using this function with the message of "ambiguous numeric type". In this case make sure that you write infront of all the values "as f64" to get rid of the possible issues.  Example (-3.5 as f64).abs() will result in a value of 3.5
*/

/* Problem - 2
In this question, we will implement the same problem as disscussed in Question 1 but wtih the help of arrays. 

Declare two arrays of names p1 and p2 both having length of 2 and type f64. These two arrays will represent two points along with their coordinate values for the  x-axis and for y-axis. Write a simple program to display the absolute difference of the x-axis and the y-axis coordinates for the two points onto the terminal.

*/


/* Problem - 4
The distance between two points p1 with coordinates (x1, y1) and p2 with coordinates (x2,y2) is computed using the formula
sqrt ((x1 - x2)^2 + (y1-y2)^2 )   where ^ = is used to indicate the exponent. Write a program which will first declare two points p1 and p2 using tuples and will Initialize the tuple p1 from the values of (4.0, 3.0) and the tuple p2 from the values of (5.0, 4.5). Next, write a statement for determining their distance and then display the result to the terminal terminal. 

Note 1: Use the function of sqrt() which is used to compute the square root of the input number. The syntax is number.sqrt().
Note 2: Use the function powf(n) which will compute the nth power of the input number. Example: (3.5 as f64).powf(2.0)  is going to computer the square of 3.5.
*/


/* Problem - 4
Chagne the  program below by writting the correct data types insted of the DATA_TYPES_PLEASE to make this program compile.

fn main() {

    let a: DATA_TYPES_PLEASE = -15;

    let b: DATA_TYPES_PLEASE = 170;

    let my_name: FIXME = "Michael";

    println!("My name is {}, and the answer is {}", my_name, a * b);

}
*/