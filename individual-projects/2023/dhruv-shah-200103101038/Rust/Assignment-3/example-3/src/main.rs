fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    let distance = (x.powf(2.0) + y.powf(2.0)).sqrt();
    distance
}
fn main() {
    println!(
        "The distance of the point from the origin is {}",
        print_distance((5.0, 4.0))
    );
}
