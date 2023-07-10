fn main() {
    println!("Enter the number:" );
    let mut number=String::new();
    std::io::stdin().read_line(&mut number).expect("Invalid input");
    let number:u32=number.trim().parse().expect("fail to enter the number");
    
  
    let mut sum:u32=0;
    let mut squre_sum:u32 =0;

    for i in 1..=number{
        sum+=i;
        squre_sum+=i.pow(2);
    }
    println!(" sum of the  all 1 to n is {}",sum);
    println!(" sum of the  all 1 to n  squre is {}",squre_sum);


    let mut diffrence = sum.pow(2)-squre_sum;


    println!("Diffrence btw sum of squre and squre of sumis {}",diffrence);
  

   

    }

   
  
/*
Write a program for finding the difference of the square of the sum and the sum of square of the first N number (where N is a user defined input that you program will take). for instance, if the user enters the number of let say 5,
Then you should first compute the squae of sum = (1+2+3+4+5)^2  = 225

and next you will compute the sum of square as  = (1^2  + 2^2  + 3^2  + 4^2  + 5^2)  = (1 + 4+ 9 + 16 +25 ) = 55

and finally you will compute the difference = 225 - 55 = 170 */



