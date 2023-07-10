fn main()
{
for a in 1..=1000{
for b in a+1..1000 {
for c in b+1..1000{ 
 if a*a + b*b == c*c && a + b+c==1000 {
                println!("\n The required pathagorian triplet are ({}, {}, {}) ", a,b,c); 
                return; 
}

}
        
}

}

}
/*
A Pythagorean triple consists of three positive integers a, b, and c, such that a*a + b*b = c*c. Such a triple is commonly written as (a, b, c), and a well-known example is (3, 4, 5). Write a program that will compute the Pythagorean triplet such that a < b < c and a+b+c = 1000. */