// calculator
// using switch case with function

// this function is addition to two value
function sum(num1, num2) {
    return num1 + num2;
  }
  // this function is subtraction to two value
  function sub(num1, num2) {
    return num1 - num2;
  }
  // this function is multiplication to two value
  function mul(num1, num2) {
    return num1 * num2;
  }
  // this function is division to two value
  function div(num1, num2) {
    return num1 / num2;
  }
  const first_number = +prompt("enter the first number", 10);
  const second_number = +prompt("enter the  second number", 20);
  const patten = prompt("enter the  opteration" ,'+');
  
  if (first_number == null && second_number == null && patten == null) {
    alert("enter the right value");
  } else {
    switch (patten) {
      case "+":
        alert(`the sum is ${sum(first_number, second_number)}`);
        break;
      case "-":
        alert(`the subtraction is ${sub(first_number, second_number)}`);
        break;
      case "*":
        alert(`the multiplication is ${mul(first_number, second_number)}`);
        break;
      case "/":
        alert(`the division is ${div(first_number, second_number)}`);
        break;
      default:
        alert("enter the right patten");
        break;
    }
  }
  