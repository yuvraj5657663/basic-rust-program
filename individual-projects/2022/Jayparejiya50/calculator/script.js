

// only using for one digit calculation

let enterValue = prompt("Enter The Expression", `2*5`);

let updatedValue = null;
let mul = function (a, b) { return a * b; }
let div = function (a, b) { return a / b; }
let add = function (a, b) { return Number(a) + Number(b); }
let sub = function (a, b) { return a - b; }

let pare = function (a, b) {

     let p_sum = null;
     if (enterValue['('] + 2 == "*") {
          return p_sum = mul(enterValue['('] + 1, enterValue['('] + 3)
     }
     else if (enterValue['('] + 2 == "/") {
          return p_sum = div(enterValue['('] + 1, enterValue['('] + 3)
     }
     else if (enterValue['('] + 2 == "+") {
          return p_sum = add(enterValue['('] + 1, enterValue['('] + 3)
     }
     else if (enterValue['('] + 2 == "-") {
          return p_sum = sub(enterValue['('] + 1, enterValue['('] + 3)
     }


}

for (let i = 0; i < enterValue.length; i++) {




     if (enterValue[i] == enterValue.indexOf('(')) {
          updatedValue = pare(enterValue[i + 1], enterValue[i + 3]);
          
     }
      if (enterValue[i] == "*") {
          updatedValue = mul(enterValue[i - 1], enterValue[i + 1]) 
     }
     else if (enterValue[i] == "/") {
          updatedValue = div(enterValue[i - 1], enterValue[i + 1])
     }
     else if (enterValue[i] == "+") {
          updatedValue = add(enterValue[i - 1], enterValue[i + 1])
     }
     else if (enterValue[i] == "-") {
          updatedValue = sub(enterValue[i - 1], enterValue[i + 1]) 
     }

}
alert(updatedValue)


