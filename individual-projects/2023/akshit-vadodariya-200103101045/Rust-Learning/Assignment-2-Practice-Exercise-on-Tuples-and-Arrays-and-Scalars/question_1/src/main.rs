fn main() {
    /* In this exercise, you will create two tuples, p1 and p2, representing points on a Cartesian plane. Each tuple will contain two values, one for the x-axis and another for the y-axis. Your task is to write a program that calculates and displays the absolute difference between the x-axis values and the absolute difference between the y-axis values.

Note: To calculate the absolute value, use the abs() function. The compiler may show an error message stating "ambiguous numeric type" when using this function. To resolve this, ensure that you specify the value as f64 by writing "as f64" in front of it. For example, (-3.5 as f64).abs() will result in a value of 3.5. */
    let _p1 = (2.2, 6.6);
    let _p2 = (4.1, 5.8);

    let _p3 = ((((_p1.0 - _p2.0) as f64).abs()), (((_p1.1 - _p2.1) as f64).abs()));
    
    println!("{:?}", _p3);
}
