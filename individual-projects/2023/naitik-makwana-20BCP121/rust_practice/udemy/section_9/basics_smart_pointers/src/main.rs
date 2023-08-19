//smart pointer can be describe as a data structure
// smart pointer is not only reference but also owner of the value

fn main() {

    //box type variable
    let val1 = Box::new(0.111);
    // value will be stored in heap but pointer which refer to data willl store on stack
    let val2 = 0.111; //this value will only store on stack
    assert_eq!(val2, *val1);
    println!("{}", val2 == *val1);

    
    //reference are always stack allocated
    let var3 = 5;
    let ref_var3 = &var3;

    let heap_var =  Box::new(var3); //heap will store copy of var3 which is on stack
    // pointer of heap_var will be on stack

    let heap_var2 = Box::new(ref_var3);
    //in this case box will points to a heap location wherestore reference to the variable var3

    let point1 = Box::new((1,2)); //box pointing to a heap loction which stores tuple
    println!(" ({},{})",point1.0,point1.1 );

    //let x1 = point //will be box type tuple
    //let x2 = *point //will be tuple only

}
