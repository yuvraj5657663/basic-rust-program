//print Fibonacci series function

// -----------with Recursion-----------------------
function fibonacci(num) {
    if (num < 2) {
        return num
    } else {
        return fibonacci(num - 1) + fibonacci(num - 2)
    }
}

function main() {
    console.log('Fibonacci Series:')
    const total_number = 10
    for (let num = 0; num < total_number; num++) {
        console.log(fibonacci(num))
    }
}
main()
/* 
work as:
   for loop is to calculate number recursively 

   as first two numbers are 0 & 1 
   0 ← step1
   1 ← step2

   next number is sum of previous two numbers as
   nth number is sum of (n-1)th number & (n-2)th number so
   (num-1) + (num - 2) = num
      ↑          ↑        ↑
      0          1        1  ← step3
      1          1        2  ← step4
      1          2        3  ← step5
      2          3        5  ← step6
      3          5        8  ← step7
      5          8        13 ← step8
      8          13       21 ← step9
      13         21       34 ← step10
*/

// -----------with temporary variable--------------

let a = 0,
    b = 1,
    temp
function logic() {
    for (let i = 1; i <= 10; i++) {
        console.log(a)
        temp = a + b
        a = b
        b = temp
    }
}
function main() {
    console.log('Fibonacci Series:')
    logic()
}
main()
