fn recieving_ownership_from() -> Vec<i32>{
    let vec1 = vec![4,5,6,9];
    vec1
}

fn main() {
    /* Consider the following simple program. When the call to the recieving_ownership_from() function completes in the main() function, who will be the owner of the vector vec1 defined inside the function?

    fn main () {
        let v1 = recieving_ownership_from();
    }

    fn recieving_ownership_from() -> Vec<i32>{
        let vec1 = vec![4,5,6,9];
        vec1
    } */

    // owner of the vectore is _v1
    let _v1 = recieving_ownership_from();
    println!("{:?}",_v1);
}
