fn some_function(a1: &String, a2: &str){
    println!("{} {}",a1,a2);
} 
fn main() {
    /* The code provided below contains a compilation error. Your task is to identify the error and make the necessary adjustments to resolve it.

    fn main(){
        let s1:String = String::from("this is me");
        let s2: &str = "myself";
        some_function(s1,s2);
        println!("{} {}",s1,s2);
    }

    fn some_function(a1: String, a2: &str){
        println!("{} {}",a1,a2);
    } */

    let s1 = String::from("this is me");
    let s2 = "myself";
    some_function(&s1, &s2);
    println!("{} {}",s1, s2)

}
