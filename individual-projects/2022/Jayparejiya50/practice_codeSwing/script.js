
/*
Ex 1: Write a function called findLargestSum that takes in an array of numbers and returns the largest sum of any two numbers in the array. For example, if the input array is [1, 2, 3, 4, 5], the output should be 9, because the largest sum is achieved by adding the numbers 4 and 5 together.
*/

function findLargestSum(arry){
     
    arry.sort();
    let a =   arry[arry.length - 1];
    let b =  arry[arry.length - 2];
    return console.log(a + b);
}

findLargestSum([1, 2, 9, 3, 5]);