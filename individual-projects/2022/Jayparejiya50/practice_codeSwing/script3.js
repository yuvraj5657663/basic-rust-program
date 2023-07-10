/* 

Ex 3: Write a function called flattenArray that takes in an array of nested arrays and returns a new array with all the elements flattened into a single level. For example, if the input array is [[1, 2], [3, 4], [5, 6]], the output array should be [1, 2, 3, 4, 5, 6].

*/


function flattenArray(nestedArray) {

    let blankArray = [];

    for (let i = 0; i < nestedArray.length; i++) {

        let innerArray = nestedArray[i];
        for (let j = 0; j < innerArray.length; j++) {
            blankArray.push(innerArray[j]);
        }


    }

    console.log(blankArray);
}

flattenArray([[1, 2], [3, 4], [10, 6]])