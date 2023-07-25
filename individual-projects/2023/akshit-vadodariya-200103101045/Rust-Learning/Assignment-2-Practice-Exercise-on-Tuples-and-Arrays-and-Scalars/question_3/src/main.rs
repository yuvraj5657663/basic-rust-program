fn main() {
    /* To calculate the distance between two points, p1(x1, y1) and p2(x2, y2), you can use the formula sqrt((x1 - x2)^2 + (y1 - y2)^2). The "^" symbol denotes exponentiation, and sqrt(___) represents the square root of the value within the parentheses.

    In this programming task, you need to write a program that accomplishes the following steps:



    1. Declare two tuples, p1 and p2, to represent the coordinates of the two points. Initialize p1 with the values (4.0, 3.0) and p2 with the values (5.0, 4.5).

    2. Write a statement to compute the distance between p1 and p2 using the given formula. To calculate the square root, use the sqrt() function, passing in the expression inside the parentheses. The syntax for computing the square root is number.sqrt().

    3. To compute the squared values within the formula, use the powf() function. This function calculates the nth power of a number. Use the syntax (number as f64).powf(2.0) to compute the square of a number.

    Display the resultant distance value to the terminal. */

    let _p1 = (4.0, 3.0);
    let _p2 = (5.0, 4.5);

    let distance = ((_p1.0 - _p2.0)as f64).powf(2.0) + ((_p1.1 - _p2.1)as f64).powf(2.0).sqrt();

    println!("The distance between p1 and p2 : {}", distance);
}
