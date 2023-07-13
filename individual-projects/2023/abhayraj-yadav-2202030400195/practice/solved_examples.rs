fn main() {
    println!("Hello, world!");
    let life = 42;
    println!("{:?} {:?}",life,life);

    let n1 = 50;
    if n1 < 100
     { 
        if  n1 > 25
        {
        println!("{} is smaller than 100 but larger than 25.",n1);
        }
        else 
        {
        println!("{} is smaller than 100.",n1);          
        }
    }
    else
    {
        println!("{} is larger than 100.",n1);
    }

     let mut num=0;
    loop {
        if num==10{
            break;
        }
        println!("{:?}",num);
        num=num+1;
    }

    let mut n=10;
    while n != 20{
        n=n+2;

        println!("{:?}",n);
    }
} 
