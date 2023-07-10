fn main() {

    //Shadowing
    let abc = 50 ;
    let abc = abc + 50 ;
    {
        let abc = abc * 2 ;
        println!("{}",abc);
    }
    println!("{abc}");


    //Numeric Operation

    let sum = 10 + 5 ;
    println!("{}",sum);

    let a = 5.0;
    let b = 10.0;
    let c = a-b;
    let d = a*b;
    let e =b/a;
    println!("{},{},{}",c,d,e);

    //Tuple 

    let tuple = (100,25,0);
    let (x,y,z) = tuple ;
    println!("y = {}",y);
    println!("x = {}, z = {} ",x,z);
    
    let last_element = tuple.2;
    println!("last element = {}", last_element);

    println!("{:?}",tuple);


    //Array

    let arr = ["apple","orenge","banana","mango"];

    let first_ele = arr[0];
    println!("{}",first_ele);

    println!("{:?}",arr);
    println!("{:?}",arr[2]);

    //function 

    
    another_function(5);


    // if else conditions
    let number = 10;
    
    if number % 2 == 0{
        println!("number is even")
    }
    else {
        println!("number is odd")
    }


    //loops

    let mut count = 1 ;
    for _i in 0..1000000 {
        println!("{:?}",count);
        count = count * 10;
        count += 1;

        if count > 1000000{
            break;
        }
        
    };


   
    
}

fn another_function(x:i32){
    let x = x + 5 ;
    println!("{x}");
}