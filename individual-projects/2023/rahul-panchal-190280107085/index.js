alert("hello this is rahul");
alert("I'm intern at codecycle");

 //if-else-if 

 // check if the number if positive, negative or zero
const number = prompt("Enter a number: ");


// check if number is greater than 0
if (number > 0) {
    console.log("The number is positive");
}
// check if number is 0
else if (number == 0) {
  console.log("The number is 0");
}
// if number is neither greater than 0, nor zero
else {
    console.log("The number is negative");
}

console.log("The if...else if...else statement is easy");


let person = {
    firstName: 'Rahul',
    middleName: 'Vishnubhai',
    lastName: 'Panchal'
}
console.log(person.lastName);
let name = 'Rahul';
let lastName = '0';
if (name === 'Rahul') {
    lastName = 'Panchal'
}
else if (name = 'r r ') {
    lastName = 'Panchal'
}
console.log(lastName);
if (+(1.1 + 1.3).toFixed(2) === 2.4) {
    showMessage('true');
}
showMessage("ternary operation = (condition)? true statement:false statement ")

if (true) {
    let value = 'very good';
    showMessage(value);
}
console.log(value)
showMessage("The above message does not work and it only if is given as var because it out side block");
for (let i = 5; i < 78; i++) {
    console.log(i);
}
let count = 4;
while (count < 15) {
    console.log(count + 4);
    count++;
}
let = 4;
do {
    console.log(Rahul + 4);
    Rahul++;
} while (Rahul < 15);

function Rahul() {
    console.log('That is my name')
    console.log('Doing intern in Codecycle')
}
let fullname = function () {
    console.log('happy to learn JS')
}
fullname();
fullname();

function codered(value) {
    let value = value * 4;
    return value;
}
console.log(codered(5));

changelastname(Patel)

let myname = secretname();
let company = {
    name: 'codecycle',
    place: 'Ahmedabad',
    salary: 10000,
    [myname]: 'Rahul Panchal',
    showInfo: function (country) {
        showMessage(this.name + 'is in' + this.place + country);
    }
};
company['salary'] = 500000;
console.log(company.name);
console.log(company.salary);
showMessage(company.place);
showMessage(company.showInfo(India));
showMessage("Document object Model(DOM)");

let now = new Date();
showMessage(now.toDateString());
showMessage(Math.abs(-78));
let R = 'Rahul';
showMessage(R.charAt(1));
const header = document.getElementById('message');
header.style.fontWeight = '332';
header.addEventListener('click', function () {
    console.log('click');
    const review = document.getElementById('review');
    if (review.classList.contains('d-list')) {
        review.classList.remove('d-list');
        header.textContent = 'close review';
    }
    else {
        review.classList.add('d-list');
        header.textContent = 'see review';
    }
});
const backend = ['Rahul Panchal', 'dhrumil', 'vrushal', 'nandini', 'ok'];
backend.push('cbum');
const last = backend.pop();
console.log(last);
const first = backend.shift();
backend.unshift('noel');
console.log(backend);
console.log(Array.isArray(backend));
//slice
const newvalue = backend.slice(2, 3);   // remove the specific array from the list
//splice
backend.splice(3, 3);    // delete the 4th name in array
backend.splice(2, 0, 'ok');    //(specify index location,delete count,insert)
const newval = backend.filter(function (item) {
    return item > 'andrie';
});
console.log(newval0);
backend.forEach(function (item) {
    console.log(item);
});

const containers = document.getElementsByClassName('container');
containers[2].classList.add('d-none');
console.log(contains);

'use strict';   //show error for below 2L and without don't - to solve use const in productId
productId = 1234;
console.log(window.productId);

showMessage('End of Getting started with JS')
