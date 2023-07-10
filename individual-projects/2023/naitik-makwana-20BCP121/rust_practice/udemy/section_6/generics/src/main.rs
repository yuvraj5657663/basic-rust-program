
// fn double<T>(x : T) -> T
// where T : std::ops::Add<Output = T> + Copy {
//     x+x
// }

// fn main() {
//     println!("Sum of all numbers : {}", double(2));
//     println!("Sum of all numbers : {:?}", double(2.2));
// }

#[derive(Debug)]
struct Point <T,U>{
    x:T,
    y:U
}

impl<T,U> Point<T,U>
where T: std::fmt::Debug + std::fmt::Display ,U: std::fmt::Debug + std::fmt::Display {
    fn printing_fun(&self){
        println!("x is : {}, y is : {}",self.x,self.y);
    }
}

fn main() {
    let p1 = Point{
        x:2,
        y:4
    };

    let p2 = Point{
        x:2.2,
        y:4.4
    };

    let p3 = Point{
        x:2,
        y:4.4
    };

    println!("p1 : {:?}", p1 );
    println!("p2 : {:?}", p2 );
    println!("p3 : {:?}", p3 );

    p1.printing_fun();
    p2.printing_fun();
    p3.printing_fun();
}