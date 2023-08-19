fn crdt() -> (i32,i32){
    (2,2)
}
fn main()
{
    let (x,y)= crdt();

    if y > 5 {
        println!("{:?}>5",y);
    }
    else if y<5
    {
        println!("{:?}<5",y);       
    }
    else
    {
        println!("{:?}=5",y);
    }
}