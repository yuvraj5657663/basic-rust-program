// Section-4 // Assignment-5

/* Problem - 1
Write a program for finding the difference of the square of the sum and the sum of square of the first N number (where N is a user defined input that you program will take). for instance, if the user enters the number of let say 5,
Then you should first compute the squae of sum = (1+2+3+4+5)^2  = 225

and next you will compute the sum of square as  = (1^2  + 2^2  + 3^2  + 4^2  + 5^2)  = (1 + 4+ 9 + 16 +25 ) = 55

and finally you will compute the difference = 225 - 55 = 170.
*/

fn main(){
     
    println!("Enter a number : ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("failure");
    let n:u32 = n.trim().parse().unwrap();

    let mut sum = 0;
    let mut sq_sum = 0;

    for i in 1..=n {
        sum+=i;
        sq_sum+= i.pow(2);
    }

    println!("sum of first {} number is : {}" , n, sum);
    println!("sum of first sqaure {} is : {}", n , sq_sum);

    let sum_sq :u32 = sum.pow(2);

    println!("Difference between both = {}", sum_sq - sq_sum );
}