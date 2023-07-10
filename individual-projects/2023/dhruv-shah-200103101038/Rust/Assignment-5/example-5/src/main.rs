// A Pythagorean triple consists of three positive integers a, b, and c,
//  such that a*a + b*b = c*c. Such a triple is commonly written as (a, b, c),
//  and a well-known example is (3, 4, 5).
//  Write a program that will compute the Pythagorean triplet such that a < b < c and a+b+c = 1000.

fn main() {
    // method -1
    // let mut input = String::new();
    // println!("Enter the sum value:");
    // std::io::stdin().read_line(&mut input).unwrap();
    // let sum: u32 = input.trim().parse().unwrap();

    // for a in 1..(sum/3) {
    //     for b in a+1..(sum/2) {
    //         let c = sum - a - b;
    //         if a*a + b*b == c*c {
    //             println!("Pythagorean triplet found: ({}, {}, {})", a, b, c);
    //             println!("Product of the triplet: {}", a * b * c);
    //             return
    //         }
    //     }
    // }

    // method -2
    for a in 1..333 {
        for b in a + 1..500 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                println!("Pythagorean triplet found: ({}, {}, {})", a, b, c);
                return
            }
        }
    }
}

// for c in 1..=1000 {
//     for b in 1..c {
//         for a in 1..b {
//             if a * a + b * b == c * c && a + b + c == 1000 {
//                 println!("Pythagorean triplet found: ({}, {}, {})", a, b, c);
//                 return;
//             }
//         }
//     }
// }
