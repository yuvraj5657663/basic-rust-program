fn main() {
    let [p1, p2] = [[3.5 as f64, 6.8 as f64], [4.2 as f64, 7.1 as f64]];
    let [x_axis, y_axis] = [(p1[0] - p2[0]).abs(), (p1[1] - p2[1]).abs()];

    println!("difference of x-axis value: {}", x_axis);
    println!("difference of y-axis value: {}", y_axis);
}
