
showMessage('help');

alert('hello world')

 // variable creation and declaration 
const name = "yash";
let age = "21";
console.log(name)

// string creation
let surname = "patel";

 // pop-up boxes 
alert(name + " " + surname);
let yourName = prompt("Enter your name");
confirm("is your name " + yourName);

// Mathematical operators with if else (calculator)

let a = +prompt("Enter first value");
let b = +prompt("Enter second value");
let operation = prompt("enter operator +, -, *, /");

//  loops
// for loop

for (let i = 0; i < 10; i++) {
    alert(i);
}

//  while loop
let j = 0;
while (j < 10) {
    console.log(j);
    j++;

}

//  do while loop
let k = 0;
do {
    console.log(k)
    k++;
} while (k < 10);

// object creation
let details = {
    name: "yash",
    age: 21,
}
console.log(details);

// functions

function hello(){
    alert("hello");
}
hello();

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
//  operator
else if (operation === "/") {
    let sum = a / b;
    alert(sum);
} else {
    alert("please enter valid number or operator");
}

//  <----arr function ------>

function arr() {
    const obj = ['yash', 'yash', 'xyz', 'abc', 'yash']
    let count = 0
    for (const iterator of obj) {
        if (iterator == 'yash') {
            count++
        }
    }
    alert(`the yash name count ${count} time `)
}

arr()

