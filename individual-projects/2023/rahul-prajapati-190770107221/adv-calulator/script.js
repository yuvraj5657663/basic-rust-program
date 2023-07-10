function appendValue(value) {
    let result = document.getElementById('result');
    
    if (value == 'sqrt(') {
    result.value += value;
    } else {
    result.value += value;
    }
  }

function clearResult() {
    document.getElementById('result').value = '';
    }


function calculate() {
    let expression = document.getElementById('result').value;
    const invalidChars = /[^0-9+\-*/().!%sqrt]|(?<!\()\)|\(\)/g;
    if (invalidChars.test(expression)) {
         console.log('Please Enter valid Input');
         return;
        }
    expression = expression.replace(/sqrt\(/g, "Math.sqrt(");
    expression = expression.replace(/%/g, "/100");
    if (expression.includes('!')) {
          let num = parseInt(expression.substring(0, expression.indexOf('!')));
          let result = factorial(num);
          document.getElementById('result').value = result;
    } else {
          var result = eval(expression);
          document.getElementById('result').value = result;
    }
 }

function factorial(num) {
     if (num == 0 || num == 1) {
       return 1;
     } else {
        return num * factorial(num - 1);
        }
 }