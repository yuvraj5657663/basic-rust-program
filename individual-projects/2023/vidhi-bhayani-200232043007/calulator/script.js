
function calculator(...numbers) {
    function sum() {
          let result = 0;
          for (let elements of numbers) {
            result += elements;
          }
          return console.log(`sum=${result}`);
    }
  
    function sub() {
          result = numbers[0];
          for (let i = 1; i < numbers.length; i++) {
              result -= numbers[i];
          }
          return console.log(`sub=${result}`);
    }
  
    function mul() {
        let result = 1;
        for (let elements of numbers) {
              result *= elements;
          }
          return console.log(`mul=${result}`);
    }
  
    function div() {
      result = numbers[0];
      if (numbers == 0) {
          return console.log("Error: Division by zero is not allowed.");
      } else {
          for (let i = 1; i < numbers.length; i++) {
              result /= numbers[i];
          }
          return console.log(`div=${result}`);
      }
    }
  
    return sum(), sub(), mul(), div();
  }
  calculator(7, 4, 4, 3);(...numbers) {
    function sum() {
          let result = 0;
          for (let elements of numbers) {
            result += elements;
          }
          return console.log(`sum=${result}`);
    }
  
    function sub() {
          result = numbers[0];
          for (let i = 1; i < numbers.length; i++) {
              result -= numbers[i];
          }
          return console.log(`sub=${result}`);
    }
  
    function mul() {
        let result = 1;
        for (let elements of numbers) {
              result *= elements;
          }
          return console.log(`mul=${result}`);
    }
  
    function div() {
      result = numbers[0];
      if (numbers == 0) {
          return console.log("Error: Division by zero is not allowed.");
      } else {
          for (let i = 1; i < numbers.length; i++) {
              result /= numbers[i];
          }
          return console.log(`div=${result}`);
      }
    }
  
    return sum(), sub(), mul(), div();
  }
  calculator(7, 4, 4, 3);