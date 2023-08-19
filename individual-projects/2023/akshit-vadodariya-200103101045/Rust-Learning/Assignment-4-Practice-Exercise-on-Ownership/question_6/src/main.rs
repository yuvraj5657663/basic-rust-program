fn get_slice(numbers: &mut Vec<i32>) -> &[i32] {
    &numbers[0..2]
}

fn main() {
    /* There is a borrowing related issue in the code below. Try identifying it?

    fn main() {
        let mut numbers = vec![1, 2, 3];
        let slice = get_slice(&mut numbers);
        numbers.push(4);
        println!("Slice: {:?}", slice);
    }

    fn get_slice(numbers: &mut Vec<i32>) -> &[i32] {
        &numbers[0..2]
    } */

    let mut numbers = vec![1,2,3];
    let slice = get_slice(&mut numbers);
    println!("Slice: {:?}", slice);
    numbers.push(4);
}
