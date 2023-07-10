// enum Result<T,E> {
//     Ok(T),
//     Err(E)
// }

fn divide(divident:f64, divisor: f64) -> Result<f64, String> {
    // if divisor == 0.0 {
    //     Err(String::from("Can not divide by zero"))
    // }else {
    //     Ok(divident/divisor)
    // }
    match divisor {
        0.0 => Err(String::from("Can not divide by zero")),
        _ => Ok(divident/divisor)
    }
}

fn main() {
    let divident = 5.5;
    let divisor = 5.0;

    println!("Answer is : {:?}", divide(divident,divisor));

    let my_vec = vec![0,1,2,3,4];
    let result = match my_vec.get(5){
        Some(a) => Ok(a),
        None => Err("value does not exists")
    };

    println!("number is : {:?}", result);
}
