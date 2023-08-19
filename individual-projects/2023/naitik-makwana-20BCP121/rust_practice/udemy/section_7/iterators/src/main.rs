fn main() {
//     let my_vec = vec![1,2,3,4,5,6,7];
//     let mut iter = my_vec.iter();

//     println!("iteration over vectoe: \n {:?}", iter);

//     println!("{}",iter.next().unwrap());
//     println!("{}",iter.next().unwrap());
//     println!("{}",iter.next().unwrap());
//     println!("{}",iter.next().unwrap());
//     println!("{}",iter.next().unwrap());
//     println!("{}",iter.next().unwrap());
//     println!("{}",iter.next().unwrap());
//     println!("{:?}",iter.next());

    // let vec1 = vec![1,2,3,4,5,6,7];

    // let mut check = vec1.iter().any(|&x| x>7); // to check any element in vector that matches the condition
    // println!("the return value of any function : {}", check);

    // let mut check = vec1.iter().all(|&x| x>0); // to check all of the element in vector that matches the condition
    // println!("the return value of all function : {}", check);

    // let check = vec1.iter().find(|&&x| x>4); // to find the element in vector that matches the condition
    // println!("the return value of find function : {}", check.unwrap());

    // let check = vec1.iter().position(|&x| x>4); // to find the position of element in vector that matches the condition
    // println!("the return value of position function : {}", check.unwrap());

    // let check = vec1.iter().rposition(|&x| x>5); // to find the position of element from the end of the svector that matches the condition
    // println!("the return value of positon function : {}", check.unwrap());

    // let check = vec1.iter().max(); // to find max
    // println!("the return value of max function : {}", check.unwrap());

    // let check = vec1.iter().max(); // to find min
    // println!("the return value of min function : {}", check.unwrap());
    
    // let check = vec1.iter().rev(); // to reverse
    // println!("the return value of rev function : {:?}", check);

    let mut vec1 =vec![1,2,3,4,5];
    let mut vec4 = vec![2,3,4,5];
    let vec2 = vec1.iter().filter(|&x| *x >= 3).collect::<Vec<_>>();
    println!("original vector : {:?}",vec1);
    println!("filtered vector : {:?}",vec2);

    let vec3 = vec4.into_iter().filter(|&x| x >= 3).collect::<Vec<_>>();
    println!("filtered vector : {:?}",vec3);

    let mut mapped = vec1.clone().iter().map(|x| 2 * *x).collect::<Vec<u32>>();
    println!("{:?}",mapped );

    let vec5 = mapped.iter().filter(|&x| *x > 4).collect::<Vec<_>>();
    println!("original vector : {:?}",mapped);
    println!("filtered vector : {:?}",vec5);

}