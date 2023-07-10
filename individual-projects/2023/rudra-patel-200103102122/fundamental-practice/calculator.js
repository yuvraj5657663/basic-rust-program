// calculator

function calculator() {
    let a = 10;
    let b = 20;
    let c = 30;
    let sum = given_sum(a, b, c);
    let subtraction = given_subtraction(a, b, c);
    let multiplication = given_multiplication(a, b, c);
    let division = given_division(a, b, c);
    console.log(
      "sum",
      sum,
      "subtraction",
      subtraction,
      "mul",
      multiplication,
      "div",
      division
    );
  }
  
  calculator();
  function given_sum(a, b, c) {
    return a + b + c;
  }
  function given_subtraction(a, b, c) {
    return a - c;
  }
  function given_multiplication(a, b, c) {
    return a * b;
  }
  function given_division(a, b, c) {
    return c / b;
  }