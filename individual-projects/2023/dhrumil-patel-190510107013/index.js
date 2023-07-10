// // variable creation and declaration 
const name = "Dhrumil";
let age = "21";

// // string creation
let surname = "patel";

// // pop-up boxes 
alert(name + " " + surname);
let yourName = prompt("Enter your name");
confirm("is your name " + yourName);

// // Mathematical operators with if else (calculator)

let a = +prompt("Enter first value");
let b = +prompt("Enter second value");
let operation = prompt("enter operator +, -, *, /");

// + operator
if (operation === "+") {
    let sum = a + b;
    alert(sum);
}
// - operator
else if (operation === "-") {
    let sum = a - b;
    alert(sum);
}

// * operator

else if (operation === "*") {
    let sum = a * b;
    alert(sum);
}
// / operator
else if (operation === "/") {
    let sum = a / b;
    alert(sum);
} else {
    alert("please enter valid number or operator");
}

// // loops
// for loop

for (let i = 0; i < 10; i++) {
    alert(i);
}

// // // while loop
let j = 0;
while (j < 10) {
    console.log(j);
    j++;

}

// // // // do while loop
let k = 0;
do {
    console.log(k)
    k++;
} while (k < 10);

// object creation


let details = {
    name: "dhrumil",
    age: 21,
}
console.log(details);

// functions

function hello(){
    alert("hello");
}
hello();
