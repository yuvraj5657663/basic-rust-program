fn main() {
    /*In this question, you will solve the same problem as discussed in Question 1, but using arrays.

    First, declare two arrays called p1 and p2. These arrays will have a length of 2 and a type of f64. Each array represents a point, with the x-axis value in the first index (position) and the y-axis value in the second index (position).

    Next, write a program that calculates and displays the absolute difference between the x-axis values and the absolute difference between the y-axis values.

    Note: Use the abs() function to determine the absolute value of the difference. In this case, you don't need to explicitly specify the types as f64 for all the values because the compiler can infer the types correctly. The compiler is smart enough to detect the types being passed to the abs() function, so you won't encounter any issue */

    let _p1 = [5.2, 2.4];
    let _p2 = [8.5, 6.8];

    let _p3 = [((_p1[0] - _p2[0])as f64).abs() ,((_p1[1] - _p2[1])as f64).abs()];

    println!("{:?}", _p3);
}
