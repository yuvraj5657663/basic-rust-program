// Section-6 // Assignment-7

/* Problem - 1
Write a program that calculates the set intersection and set union of two vectors containing some numbers. The program should have two separate functions: one for calculating the intersection and another for calculating the union.

The program will define two input vectors, vec_1 and vec_2, which contain unsigned 32-bit integers. For example:
let mut vec_1: Vec<u32> = vec![5, 4, 3, 6, 9];
let mut vec_2: Vec<u32> = vec![5, 8, 6, 4, 10, 15, 20, 21];

These vectors will be passed as arguments to the intersection and union functions to compute the respective results.
*/

fn intersection(vec1:&Vec<u32>, vec2 : &Vec<u32>) -> Vec<u32> {
    let mut outvec = Vec::new();

    for i in vec1 {
        for j in vec2 {
            if i==j {
                outvec.push(*i);
            }
        }
    }
    outvec
}

fn union(vec1:&Vec<u32>, vec2 : &Vec<u32>) -> Vec<u32> {
    let mut outvec = Vec::new();

    for i in vec1 {
        outvec.push(*i);
    }

    for j in vec2 {
        if outvec.contains(j)==false {
            outvec.push(*j);
        }
    }

    outvec
}

fn main() {

    let v1 = vec![1,2,3,4,5];
    let v2 = vec![3,4,5,6,7];
    println!("vector 1 : {:?}",v1);
    println!("vector 2 : {:?}",v2);
    println!("intersection : {:?}",intersection(&v1,&v2));
    println!("union : {:?}",union(&v1,&v2));
}
