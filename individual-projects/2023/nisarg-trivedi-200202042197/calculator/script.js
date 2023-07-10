// //calculator



alert("Select operation.");
alert("1.+. Add");
alert("2.-. Subtract");
alert("3.*. multiplication");
alert("4./. Divide");

const a = +prompt("enter first value");
const b = +prompt("enter second value");
const choise = prompt("enter choise(+,-,*,/): ");

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

// operation if else statement

if(choise == "+"){
    alert(add(a,b));
}

else if (choise == "-") {

    alert(sub(a,b));
}

else if (choise == "*") {

    alert(mul(a,b));
}

else if (choise =="/") {

    alert(div(a,b));

} 
else {
    alert("Error: Please Enter a Valid Number");
}
