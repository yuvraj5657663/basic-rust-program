// ====================== function ======================
// fibonacci series

function main(){
    let number = 10;
    let number1 = 0;
    let number2 = 1;
    var next_number = 0;
    console.log("the fibonacci series is: ");
    for (let i = 0; i < number; i++) {
        if (i <= 1) {
            next_number = i;
        }
        else {
            next_number = number1 + number2;
            number1 = number2;
            number2 = next_number;
        }
        console.log("{}", next_number);
    }
}




// when num=0 than go fibonacci(num) it's <2 than return 0
// when num=1 than go fibonacci(num) it's <2 than return 1
// when num=2 than go fibonacci(num) it's >2 than return 1
// when num=3 than go fibonacci(num) it's >2 than fibonacci (3-1) + fibonacci (3-2) 
// fibonacci(2)+fibonacci(1);
// return 1+1 = 2
// when num=4 than go fibonacci(num) it's >2 than fibonacci (4-1) + fibonacci (4-2)
// fibonacci(3)+fibonacci(2);
// return 2+1 = 3
// when num=5 than go fibonacci(num) it's >2 than fibonacci (4-1) + (4-2)
// fibonacci(4)+fibonacci(3);
// return 3+2 = 5

// when num=3 than go fibonacci(num) it's >2 than fibonacci (4-1) + (4-2) return 5+8 = 13
// when num=3 than go fibonacci(num) it's >2 than fibonacci (4-1) + (4-2) return 8+13 = 21
// when num=3 than go fibonacci(num) it's >2 than fibonacci (4-1) + (4-2) return 13+21 = 34






// function fibonacci(number) {
//     if(number < 2) {
//         return number;
//     }
//     else {
//         return fibonacci(number-1) + fibonacci(number - 2);
//     }
// }

// const n_number = prompt('Enter the number of terms: ');

// if(n_number <=0) {
//     console.log('Enter a positive integer.');
// }
// else {
//     for(let i = 0; i < n_number; i++) {
//         console.log(fibonacci(i));
//     }
// }