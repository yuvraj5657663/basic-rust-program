struct Nums {
    some_data : Vec<u32>
}

trait BasicOp {
    fn sum(&self) -> u32;
    fn avg(&self) -> f32;

}

impl BasicOp for Nums {
    fn sum(&self) -> u32 {
        let mut sum = 0;
        for i in self.some_data.iter() {
            sum += *i;
        }
        sum
    }

    fn avg(&self) -> f32 {
        self.sum() as f32 / self.some_data.len() as f32
    }
}

fn main() {

    let my_vec:Vec<u32> =  vec![1,2,3,4,5,6,7,8,9,10]; 
    let my_vec_struct = Nums {
        some_data : my_vec
    };

    println!("sum of numbers in vector : {}", my_vec_struct.sum());
    println!("avg of number in vector : {}", my_vec_struct.avg());

}
