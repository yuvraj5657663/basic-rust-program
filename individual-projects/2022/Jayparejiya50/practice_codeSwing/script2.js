/*
    Ex 2: Write a function called countOccurrences that takes in an array and a value, and returns the number of times the value appears in the array. For example, if the input array is [1, 1, 2, 3, 3, 4, 5, 5] and the input value is 1, the output should be 2.
*/


function countOccurrences(array,a){

let result = array.filter(array => array==5);
let count = result.length;

return console.log(count);

}

countOccurrences([1, 1, 2, 3, 3, 4, 5, 5],5)