/*
Ex 4: Write a function called fibonacci that takes in a number n and returns an array of the first n numbers in the Fibonacci sequence. The Fibonacci sequence is a series of numbers in which each number is the sum of the two preceding ones, starting from 0 and 1.

*/
function fibonacci(n) {
    let emptyArray = [0];
    let num1= 0;
    let num2=1;
    let num3;
    for (let i = 0; i < n; i++) {

        num3 = num1 + num2;
        num2 = num1;
        num1 = num3;

        emptyArray.push(num3);
    }

    console.log(emptyArray)
}


fibonacci(8);