static PI : f32 = 3.14;

struct Rectangle {
    length : u32,
    width :  u32
}

struct Circle {
    radius : u32
}

struct Square {
    length : u32
}

trait Area {
    fn shape_area(&self) -> f32 {
        println!("not implemented for this type");
        0.0 // it will show this line if the fucntion is not implemented for entered type
        //default implementation
    }
    fn shape_perimeter(&self) -> f32;
}

impl Area for Rectangle {
    fn shape_area(&self) -> f32 {
        (self.length as f32) * (self.width as f32)
    }

    fn shape_perimeter(&self) -> f32 {
        2.0*(self.length as f32) + 2.0*(self.width as f32)
    }
}

impl Area for Circle {
    fn shape_area(&self) -> f32 {
        PI * (self.radius as f32).powf(2.0)
    }

    fn shape_perimeter(&self) -> f32 {
        2.0*PI*(self.radius as f32)
    }
}

impl Area for Square {
    fn shape_area(&self) -> f32 {
        (self.length as f32).powf(2.0)
    }

    fn shape_perimeter(&self) -> f32 {
        4.0*(self.length as f32)
    }
}



fn main () {
    let rectangle_1 = Rectangle {
        length : 5,
        width : 4
    };

    let circle_1 = Circle {
        radius : 5
    };

    let square_1 = Square {
        length : 5
    };


    println!("Area of rectangle is : {} and perimeter is : {}", rectangle_1.shape_area(), rectangle_1.shape_perimeter());
    println!("Area of circle is : {} and circumference is : {}", circle_1.shape_area(), circle_1.shape_perimeter());
    println!("Area of square is : {} and perimeter is : {}", square_1.shape_area(), square_1.shape_perimeter());
}