// issue 03 Calculator with functions

const a = +prompt("enter first value");
const b = +prompt("enter second value");
const c = prompt("enter operator +,-,*,/");

function add(a, b){
    return a+b;
}
function sub(a, b){
    return a-b;
}
function mul(a, b){
    return a*b;
}
function div(a, b){
    return a/b;
}

// + operator
if(c==="+"){
    alert(add(a,b));
}
// - operator
else if (c === "-") {
    alert(sub(a,b));
}

// * operator
else if (c === "*") {
    
    alert(mul(a,b));
}

// / operator
else if (c === "/") {
   
    alert(div(a,b));
} else {
    alert("please enter valid number or operator");
}