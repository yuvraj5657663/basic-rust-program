fn some_print(){
    println!("**********");
}

pub mod maths {
    pub mod basic_math {
        
        pub fn add(num1:&i32,num2:&i32) -> i32 {
            num1+num2
        }

        pub fn sub(num1:&i32,num2:&i32) -> i32 {
            num1-num2
        }

        pub fn multiply(num1:&i32,num2:&i32) -> i32 {
            let ans = num1*num2;
            printing(&ans);
            ans
        }

        //private function can be called inside module
        fn printing(ans:&i32) {
            println!("the result is : {} // printed using private fucntion inside module",ans);
            crate::crate_1::some_print();
        }
    }
}


pub fn rectangle_area(length:&i32,width:&i32) -> i32 {
    use maths::basic_math::multiply;
    multiply(length,width)
}

pub fn circle(rad:&i32) -> f32 {
    3.14 * (*rad as f32).powf(2.0)
}
