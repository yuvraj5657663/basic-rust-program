//Topic : Structures

/*
Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.

*/


#[derive(Debug)]

struct Point {
    x: f32,
    y: f32,
}



#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

//task 1
fn rect_area(r: Rectangle) -> f32 {

    let topleft = r.top_left;
    let bottomright = r.bottom_right;

    let height = topleft.y - bottomright.y;
    let width = bottomright.x - topleft.x;

    let area:f32 = height * width;
    area
}

//task 2
fn square(p:Point, a: f32) -> Rectangle {

    let height:f32 = a.sqrt();
    let top_left:Point = p;
    let bottom_right_x = height + top_left.x ;
    let bottom_right_y = top_left.y - height;

    let new_square = Rectangle{
        top_left : top_left,
        bottom_right: Point {x: bottom_right_x, y:bottom_right_y},
    };

    new_square
}

fn main() {

    let point: Point = Point { x: 4.3, y: 5.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 10.2, y: 3.2  };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let s_topleft_point: Point = Point { x: 4.3, y: 5.4 };
    println!("s_point: ({}, {})", s_topleft_point.x, s_topleft_point.y);

    let Point { x: left_edge, y: top_edge } = point;

    let rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    println!("{}",rect_area(rectangle));
    println!("{:#?}",square(s_topleft_point,16.0));

}