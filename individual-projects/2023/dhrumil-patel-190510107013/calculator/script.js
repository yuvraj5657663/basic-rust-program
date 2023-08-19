//addition 

function add(num1, num2) {
    return parseInt(num1) + parseInt(num2);
}

//Subtraction 

function sub(num1, num2) {
    return num1 - num2;
}

//Division 

function div(num1, num2) {
    return num1 / num2;
}

//Multiplication 

function mul(num1, num2) {
    return num1 * num2;
}


const firstNumber = prompt("Enter the first number:", );
const operator = prompt("Enter the operator:");
const secondNumber = prompt("Enter the second number:", );

if (firstNumber == null && secondNumber == null && operator == null) {
    alert("Enter the right value !");
}
else {
    switch (operator) {
        case "+":
            alert(`Sum is ${add(firstNumber, secondNumber)}`);
            break;
        case "-":
            alert(`Subtraction is ${sub(firstNumber, secondNumber)}`);
            break;
        case "*":
            alert(`Multiplication is ${mul(firstNumber, secondNumber)}`);
            break;
        case "/":
            alert(`Division is ${div(firstNumber, secondNumber).toFixed(4)}`);
            break;
        default:
            alert("enter the right operator :");
            break;
    }
}