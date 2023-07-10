//addition of 2 values
function add(num1, num2) {
    return parseInt(num1) + parseInt(num2);
}

//Subtraction of 2 values
function sub(num1, num2) {
    return num1 - num2;
}

//Multiplication of 2 values
function mul(num1, num2) {
    return num1 * num2;
}

//Division of 2 values
function div(num1, num2) {
    return num1 / num2;
}

const first_number = prompt("Enter the first number:", 10);
const second_number = prompt("Enter the second number:", 20);
const operator = prompt("Enter the operator:");

if (first_number == null && second_number == null && operator == null) {
    alert("Enter the right value !");
}
else {
    switch (operator) {
        case "+":
            alert(`Sum is${add(first_number, second_number)}`);
            break;
        case "-":
            alert(`Subtraction is ${sub(first_number, second_number)}`);
            break;
        case "*":
            alert(`Multiplication is${mul(first_number, second_number)}`);
            break;
        case "/":
            alert(`Division is ${div(first_number, second_number).toFixed(2)}`);
            break;
        default:
            alert("enter the right operator:");
            break;
    }
}

