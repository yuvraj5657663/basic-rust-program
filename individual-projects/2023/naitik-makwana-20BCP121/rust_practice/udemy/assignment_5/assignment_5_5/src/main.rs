// Section-4 // Assignment-5

/* Problem - 5
   A Pythagorean triple consists of three positive integers a, b, and c, such that a*a + b*b = c*c. Such a triple is commonly written as (a, b, c), and a well-known example is (3, 4, 5). Write a program that will compute the Pythagorean triplet such that a < b < c and a+b+c = 1000.
*/

fn main(){
    
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for x in 1..500{
        for y in (x+1)..500 {
            
            let z:i32 = 1000-x-y;

            if x<y && y<z {
                if x+y+z==1000 {
                    if (x as i32).pow(2) + (y as i32).pow(2) == z.pow(2) {
                        a=x;
                        b=y;
                        c=z;
                    }
                }
            }
            
        }
    }

    println!("a:{}, b:{}, c:{}",a,b,c );
}