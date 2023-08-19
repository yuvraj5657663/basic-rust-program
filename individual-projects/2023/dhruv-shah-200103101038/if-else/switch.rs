fn main(){
    let no_of_switch = 5;
    let mut switch_work = 0;
    let mut switch_not_work = 0;

    for i in no_of_switch{
        if (switch_is_working == true){
            println!("it is working status");
            switch_work += 1;
         }
         else {
             println!("it is not working status");
             switch_not_work += 1;
         }
    }
   
    println!("number of work switches :{}", switch_work);
    println!("number of not work switches :{}", switch_not_work);
}