fn main() {
    let (p1, p2) = ((4.0 as f64, 3.0 as f64), (5.0 as f64, 4.5 as f64));
    let distance = ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt();
    println!("Distance between p1 and p2: {}", distance);
}