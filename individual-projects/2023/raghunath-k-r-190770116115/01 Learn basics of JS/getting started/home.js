alert('hello world')
alert('This K R Raghunath, How are you sir.')

let price = 20,
    product = 'Codecycle',
    discounted = true;
    showMessage(--price);
product=product.toLowerCase();
console.log(price);
showMessage("we are happy to work as intrn in Codecycle!!!!");
console.log("I am happy to learn new programming language Javascript !!!! ........This messgage is shown in website>properties>console");
showMessage("always start with _,$,letter to create variable and assigning variable should be fist then consle and showMessage all.");
showMessage("MDN- mozila developer network")
let amount = Number.parseFloat("332.78");
showMessage(typeof amount);
let saved = true;
saved=!saved;
saved=null;
showMessage(saved);
console.log(saved);
let person={
    firstName : 'K',
    middleName:'R',
    lastName:'Raghunath'
}
console.log(person.lastName);
let name = 'Raghu';
let lastName= '0';
if(name === 'Raghu'){
    lastName='nath'
}
else if (name='K R'){
    lastName='Raghunath'
}
console.log(lastName);
 if (+(1.1 + 1.3).toFixed(2) === 2.4){
    showMessage('true');
 }
showMessage("ternary operation = (condition)? true statement:false statement ")

if (true){
    let value = 'Good';
    showMessage(value);
}
console.log(value)
showMessage("The above message does not work and it only if is given as var because it out side block");
for(let i=5;i<78;i++){
    console.log(i);
}
let count = 4;
while(count<15){
    console.log(count+4);
    count++;
} 
let KR = 4;
do{
    console.log(KR+4);
    KR++;
}while(KR<15);

function Raghunath(){
    console.log('That is my name')
    console.log('Doing intern in Codecycle')
}
let fullname=function(){
    console.log('happy to learn JS')
}
fullname();
fullname();

function codered(value){
    let value=value*4;
    return value;
}
console.log(codered(5));

changelastname(Gounder)

let myname= secretname();
let company={
    name : 'codecycle',
    place:'Ahmedabad',
    salary: 100000,
    [myname]:'K R Raghunath',
    showInfo: function(country){
        showMessage(this.name + 'is in'+ this.place+country);
    }
};
company['salary']=500000;
console.log(company.name);
console.log(company.salary);
showMessage(company.place);
showMessage(company.showInfo(India));
showMessage("Document object Model(DOM)");

let now = new Date();
showMessage(now.toDateString());
showMessage(Math.abs(-78));
let R = 'Raghunath';
showMessage(R.charAt(1));
const header = document.getElementById('message');
 header.style.fontWeight ='332';
 header.addEventListener('click',function(){
    console.log('click');
    const review = document.getElementById('review');
    if(review.classList.contains('d-list')){
    review.classList.remove('d-list');
    header.textContent='close review';
    }
    else{
        review.classList.add('d-list');
        header.textContent='see review';  
    }
 });
 const backend =['Raghu','surya','Khushi','Divya','Deepdevani'];
  backend.push('Sakshi');
  const last = backend.pop();
  console.log(last);
  const first = backend.shift();
  backend.unshift('Jayesh');
  console.log(backend);
  console.log(Array.isArray(backend));
//slice
 const newvalue= backend.slice(2,3);   // remove the specific array from the list
 //splice
 backend.splice(3,3);    // delete the 4th name in array
 backend.splice(2,0,'sonu');    //(specify index location,delete count,insert)
 const newval = backend.filter(function(item){
    return item>'surya';
 });
 console.log(newval0);
 backend.forEach(function(item){
    console.log(item);
 });

 const containers = document.getElementsByClassName('container');
  containers[2].classList.add('d-none'); 
  console.log(contains);
 
'use strict';   //show error for below 2L and without don't - to solve use const in productId
productId =1234;
console.log(window.productId);
 
showMessage('End of Getting started with JS');


showMessage('starting syntax and operator');
showMessage('in html file');

function simplertrycatch(){
    let result;
    try {
        result=x/10;
     } catch(error){
        console.log(error.message);
     }
}