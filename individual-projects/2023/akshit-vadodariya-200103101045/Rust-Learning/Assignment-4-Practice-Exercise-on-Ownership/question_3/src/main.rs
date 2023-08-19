fn recieving_giving_back_ownership(vec1: Vec<i32>) -> Vec<i32> {
    vec1
}

fn main() {
    /* Consider the following program given below. Who owns the vector containing the values of [5, 8, 9, 7] at the end of the main function, when the function call to the function recieving_giving_back_ownership() completes

    fn main(){
        let v2 = vec![5, 8,9,7];
        let v3 = recieving_giving_back_ownership(v2);
    }

    fn recieving_giving_back_ownership(vec1: Vec<i32>) -> Vec<i32>{
        vec1
    } */

    // _v3 owns the vector containing the value at the end of the main  function
    let v2 = vec![5,8,9,7];
    let mut _v3 = recieving_giving_back_ownership(v2);
    //becouse :- 
    _v3.push(12);
    println!("{:?}", _v3);
}
