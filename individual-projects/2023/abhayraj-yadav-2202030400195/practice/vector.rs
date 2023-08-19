
fn main()
{
    let my_num = vec![10,20,30,40];
    
    for num in &my_num
    {
        match num 
        {
        30 => println!("Thirty"),
        _  => println!("{:?}",num)
        }
    }
    println!("Number of elements = {:?}",my_num.len());
}