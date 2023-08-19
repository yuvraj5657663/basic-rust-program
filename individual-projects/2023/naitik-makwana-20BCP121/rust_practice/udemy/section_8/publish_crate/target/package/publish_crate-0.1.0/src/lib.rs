//! # Basic math crate
//! This is a collection of some genrally used math fucntions
//! 
/// to compute a square of given number
/// 
/// # Examples
/// let x = 2;
/// println!("square is : {}", square(x));
/// 
/// output :
/// square is : 4
/// 
/// # Tests
/// ```
/// let n = 5;
/// let ans = publish_crate::square(n);
/// 
/// assert_eq!(25,ans);
/// ```
/// # Limitation
/// only for i32 type integers
/// 
/// # Other Details
/// etc etc
/// 


pub fn square(num:i32) -> i32 {
    num*num
}

/// to compute a cube of given number
/// 
/// # Examples
/// let x = 2;
/// println!("cube is : {}", cube(x));
/// 
/// output :
/// square is : 8
/// 
/// # Tests
/// ```
/// let n = 5;
/// let ans = publish_crate::cube(n);
/// 
/// assert_eq!(125,ans);
/// ```
/// # Limitation
/// only for i32 type integers
/// 
/// # Other Details
/// etc etc
/// 
pub fn cube(num:i32) -> i32 {
    num*num*num
}

/// to compute a double of given number
/// 
/// # Examples
/// let x = 2;
/// println!("double is : {}", cube(x));
/// 
/// output :
/// square is : 4
/// 
/// # Tests
/// ```
/// let n = 5;
/// let ans = publish_crate::double(n);
/// 
/// assert_eq!(10,ans);
/// ```
/// # Limitation
/// only for i32 type integers
/// 
/// # Other Details
/// etc etc
/// 
pub fn double(num:i32) -> i32 {
    num*2
}