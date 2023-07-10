fn main() {
    println!("Distance from origin is {}", print_distace((3.0,5.0)));
    
    
}
fn  print_distace( point :(f32,f32)) -> f32 {
    let (x,y) = point;
    ( x.powf(2.0) + y.powf(2.0) ).sqrt()
  

}
/*
Write a function which will accept a tuple called point representing the x-axis and y-axis coordinates of a point. The function will compute the distance of the point from the origin and will return the computed distance. */