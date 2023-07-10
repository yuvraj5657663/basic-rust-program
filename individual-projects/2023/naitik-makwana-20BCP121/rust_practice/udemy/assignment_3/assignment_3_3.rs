// Section-2 // Assignment-3

/* Problem - 3
Write a function which will accept a tuple called point representing the x-axis and y-axis coordinates of a point. The function will compute the distance of the point from the origin and will return the computed distance.
*/


fn print_distance(point: (f32, f32)) -> f32{ 
    
    ((point.0 as f32).powf(2.0) + (point.1 as f32).powf(2.0)).sqrt()
}

fn main() {
    println!("{}", print_distance((2.0,2.0)));
}
