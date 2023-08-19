// ARRAYS !!!
// ARRAYS ARE THE COLLECTION OF ELEMENTS THAT ARE OF SAME DATA TYPE :   
fn main() {

let mut  number_array : [i32;5]= [1,2,3,4,5];
// to print any of the array element!
println!("{ }", number_array[0]);

// to print all the elements os array!
println!("{:? }", number_array);

// to update any of the value in array :
number_array[4] = 6;

println!("{:? }", number_array);    

// TO GIVE REFERENCE  
// here subset_array is printing first three values of number_array 
// note -  we cant update such values as they are the refrences !!!
let subset_array = &number_array[0..=2];
println!("{:? }",subset_array);

// to check that how many bytes are occupied by the array  !!
println!("the array is occupaying { }bytes" , std :: mem :: size_of_val(&number_array));

// to check the length / size of the array !!
println!("{}", number_array.len());

// to check the index (value) we can use (dot).get function !!
let check_index = number_array.get(1);
println!("{:?}",check_index);
// if the index does not exists then compiler will return value [none] !!

}