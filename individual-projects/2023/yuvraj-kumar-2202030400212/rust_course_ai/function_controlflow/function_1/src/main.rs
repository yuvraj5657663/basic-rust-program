// simple addition

// fn add(a: i32, b: i32) -> i32{
//     a+b
// }
// fn main(){
//     let sum = add(5,3);
//         println!("sum {} ", sum);
//     }

// absoulte value 

// fn absolute_value (x: i32) -> i32{
//     if x >= 0 {
//         x
//     }else {
//         -x
//     }
// }
// fn main (){
//     let num = -7;
//     let abs_num = absolute_value(num);
//     println!("absoute value of {}: {}", num, abs_num);

// }


// factorial using recursion

// fn factorial(n: u32) -> u32 {
//     if n == 0 {
//         1
//     }else {
//         n * factorial(n-1)
//     }
// }
// fn main(){
//     let num = 5;
//     let fact = factorial(num);
//     println!("factorial of {}: {}", num , fact);

// }


//  fn max(a: i32, b: i32) -> i32 {
//     if a> b{
//         a 
//     }else{
//         b
//     }
// }
// fn main (){
//     let num1 = 10;
//     let num2 = 7;
//     let max_num = max(num1, num2);
//     println!("Max of {} and {}: {}", num1, num2, max_num);

// }


// fibonacci series

// fn fibonacci(n: u32) -> u32 {
//     if  n <= 1 {
//         n 
//     }else{
//         fibonacci(n-1)+fibonacci(n-2)
//     }
// }
// fn main(){
//     let term = 43;
//     let fib = fibonacci(term);
//     println!("fibonacci term {} is: {}", term,fib);
// }


// simple calculator using function

// fn add(a: f64, b:f64 ) -> f64 {
//     a+b
// }

// fn subtract(a: f64, b: f64) -> f64{
//     a-b
// }
// fn multiply (a:f64, b: f64) -> f64{
//     a*b
// }
// fn divide(a: f64, b:f64) -> f64{
//     if b != 0.0 {
//         a/b
//     }else{
//         panic!("division by zero!");

//     }
    
// }

// fn main(){
//     let num1 = 10.0;
//     let num2 = 5.0;

//     let sum = add(num1,num2);
//     let difference = subtract(num1, num2);
//     let product = multiply(num1,num2);
//     let quotient = divide(num1, num2);

//     println!("sum: {}", sum);
//     println!("diffrence: {}", difference);
//     println!("product: {}", product);
//     println!("quotient: {}", quotient);
// }

 

// celsius to fahernheit converter

// fn celsius_to_fahrenheit(celsius: f64) -> f64 {
//     (celsius * 9.0/5.0) + 32.0
// }
// fn main(){
//     let celsius_temp = 37.0;
//     let fehrenheit_temp = celsius_to_fahrenheit(celsius_temp);

//     println!("{}°C is {}°F", celsius_temp, fehrenheit_temp);

// }


//positive or negative check number

// fn positive_or_negative(num:i32){
//     if num> 0{
//         println!("positive");

//     }else if num < 0 {
//         println!("negative");
//     }else {
//         println!("Zero");
//     }
// }

// fn main(){
//     let number = -7;
//     positive_or_negative(number);
// }


// largest element in an array

// fn find_largest(numbers: &[i32]) -> i32 {
//     let mut largest = numbers[0];

//     for &num in numbers {
//         if num > largest {
//             largest = num; 
//         }
//     }
//     largest
// }
// fn main(){
//     let numbers = [10,7,21,3,15];
//     let max_num = find_largest(&numbers);
//     println!("largest element: {}", max_num);

// }


// print fibonnacci series

// fn fibonacci_series(n: u32){
//     let mut a = 0;
//     let mut b = 1;
//     for _ in 0..n {
//         print!("{} ",a);
//         let next = a+b;
//         a=b;
//         b=next;
//     }
// }
// fn main(){
//     let terms = 8;
//     println!("fibonacci series of {} terms: ", terms);
//     fibonacci_series(terms);
// }


// prime number checker

// fn is_prime(n: u32) -> bool {
//     if n <= 1 {
//         return false;

//     }

//     for i in 2..=(n/2) {
//         if n%i == 0 {
//             return false;
//         }
//     }
//     true
// }
// fn main(){
//     let num = 17;
//     if is_prime(num) {
//             println!("{} is a prime number", num);
//         } else{
//             println!("{} is not a prime number ", num);

//         }
    
// }

// calculate factorial(iterative) 

// fn factorial_iterative(n: u32) -> u32 {
//     let mut result = 1;
//     for i in 1..=n {
//         result *= i;

//     }
//     result
// }
// fn main (){
//     let num = 5;
//     let fact = factorial_iterative(num);
//     println!("factorial of {} (iterative): {}", num , fact);
    
// }


// power calculation 

// fn power ( base: f64, exponent : i32) -> f64{
//     if exponent ==0 {
//         1.0
//     }else if exponent > 0 {
//         base * power(base, exponent -1 )
//     }else {
//         1.0 / (base *power(base, -exponent -1))
//     }
// }
// fn main(){
//     let base = 2.0;
//     let exponent = 3;
//     let result = power(base, exponent);
//     println!("{} ^ {} = {}", base, exponent,result);
// }



// palindrone checker

// fn is_palindrone(s: &str) -> bool {
//     let reversed: String = s.chars().rev().collect();
//     s == reversed
// }
// fn main(){
//     let word = "level";
//     if is_palindrone(word){
//         println!("{} is a palindrone", word);

//     }else{
//         println!("{} is not a palindrome",word);
//     }
// }



// GCD and LCM calculation

// fn gcd(a: u32, b: u32)-> u32{
//     if b==0{
//         a 
//     }else{
//         gcd(b,a%b)

//     }
// }
// fn lcm(a: u32,b:u32) -> u32 {
//     (a * b) /gcd (a,b)
// }

// fn main(){
//     let num1 = 12;
//     let num2 = 18;
//     let gcd_value = gcd(num1,num2);
//     let lcm_value = lcm(num1,num2);
//     println!("GCD of {} and {}: {}", num1,num2,gcd_value);
//     println!("LCM OF {} and {}: {}",num1,num2,lcm_value);
// }


// print multiplication table

// fn multiplication_table(num: u32){
//     for i in 1..=10{
//         println!("{} x {} = {}", num,i,num*i);

//     }
// }

// fn main(){
//     let number = 7;
//     println!("multiplication table of {}: ",number);
//     multiplication_table(number);
// }


// table of 10

// fn multiplication_table(num:u32){
//     for i in 1..=10{
//         println!("{} x {} = {}", num, i, num*i);

//     }
// }
// fn main(){
//     let number = 10;
//     println!("multiplication table of {}: ",number);
//     multiplication_table(number);
// }

// reverse a nnumber

// fn reverse_number(num: u32) -> u32 {
//     let mut n = num;
//     let mut reversed = 0;
//     while n> 0{
//     let digit = n%10;
//     reversed = reversed * 10 +digit;
//     n /=10;
//     }
// reversed
// }

// fn main(){
//     let num = 124345;
//     let reversed = reverse_number(num);
//     println!("reverse of {}: { }  ",num,reversed);
// }


// // count digit a numbers
// fn count_digits(num: u32) -> u32 {
//     let mut n = num;
//     let mut count = 0;
//     while n > 0 {
//         n /=10;
//         count +=1;

//     }
//     count
// }

// fn main(){
//     let num = 9876;
//     let digit_count = count_digits(num);
//     println!("Number of digit in {}: {}",num,digit_count);
// }

// same example of practice

// fn count_digit(num: u32)->u32{
//     let mut n = num;
//     let mut count = 0;
//     while n> 0{
//         n /= 10;
//         count += 1;

//     }
//     count

// } 
// fn main(){
//     let num = 973864679;
//     let digit_count = count_digit(num);
//     println!("number of digits in {}: {}", num,digit_count);
// }


// check armstrong number

// fn is_armstrong_number(num: u32) -> bool {
//     let mut n = num;
//     let mut sum = 0;
//     let digits = count_digit(num); // error here line
//     while n > 0 {
//         let digit = n % 10;
//         sum += digit.pow(digits);
//         n /= 10;
//     }
//     num == sum
// }

// fn main() {
//     let num = 153;
//     if is_armstrong_number(num) {
//         println!("{} is an Armstrong number", num);
//     } else {
//         println!("{} is not an Armstrong number", num);
//     }
// }



// calculate sum array elements
fn sum_array_elements(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        sum +=num;

    }
    sum
}

fn main(){
    let numbers = [10,5,3,8,12];
    let total = sum_array_elements(&numbers);
    println!("sum of array elements: {}",total);
}