// Section-4 // Assignment-5

/* Problem - 2
Find the sum of natural numbers below number N (where N is provide by user) that are multiples of either 3 or 5.  For example, if the user enters a number N = 20 then

multiples of 3 = 3,6,9,12,15,18

multiples of 5 = 5, 10, 15

Sum = 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18   (Please note that value of 15 will be counted once since it is multiple of both 3 and 5)
*/
fn main() {
 
    println!("Enter a number : ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("failure");
    let n:u32 = n.trim().parse().unwrap();

    let mut sum : u32 = 0;

    for i in 1..n {
        if i%3==0 || i%5==0 {
            sum+=i;
        }
        else {

        }
    }

    println!("sum of natural numbers below number {} that are multiples of either 3 or 5 = {}", n, sum);
}