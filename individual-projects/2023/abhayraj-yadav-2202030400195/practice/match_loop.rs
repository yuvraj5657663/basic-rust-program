fn main()
{
    let check_value ='3';
    match check_value {
        '1' => println!("one."),
        '2' => println!("two."),
        '3' => println!("three."),
        _ => println!("other.")
    }
}fn main()
{
    let mut x = 5;
    loop {
        println!("({:?}) HELLO",x);
        x=x-1;
        if x==0{
            break;
        }
    }
    println!("DONE!!!!!")
}fn main()
{
    let mut a = 0.5;

    while a<=4.5 {
        
        println!("{:?}",a);
        a = a + 0.5;
        
    }
}