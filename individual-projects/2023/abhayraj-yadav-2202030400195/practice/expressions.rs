
//DEMO
/* 
enum Access{
    ADMIN,
    MANAGER,
    GUEST
}
 fn main()
 {
    let access_file = Access::GUEST;
    let can_access = match access_file{
        Access::ADMIN => true,
                 _    => false
    };
    println!("can access? : {:?}",can_access);
 }
 */
fn number_is(gt_100:bool){
    match gt_100{
        true => println!("its big."),
        false=> println!("its small.")
    };
}
fn main(){
    let number = 100;
    let check = number > 100;
    number_is(check);

}
