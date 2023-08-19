//topic : Display

/*
After checking the output of the above example, use the Point2D struct as a guide to add a Complex struct to the example. When printed in the same way, the output should be:

Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
 */
use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}


impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {

    let point = Complex { real: 3.3, imag: 7.2 };

    println!("Display: {}", point);
    println!("Debug: {:?}", point);

}