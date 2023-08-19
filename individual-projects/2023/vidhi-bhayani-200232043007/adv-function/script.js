
function factorial(n) {

    let factorial = 1;

    for (let i = 1; i <= n; i++){

        factorial = factorial * i;
    }
    return console.log(`factorial = ${factorial}`);
}

factorial(5);


//Output a single-linked list

let list = {
    value: 1,
    next: {
      value: 2,
      next: {
        value: 3,
        next: {
          value: 4,
          next: null
        }
      }
    }
  };
function printList(list) {
    console.log("printListss(list) = ");
    while (list) {
      console.log(list.value);
      list = list.next;
    }
  }

printList(list);

//Sum with an arbitrary amount of brackets

function sum(a) {

    let currentSum = a;

    function f(b) {
      currentSum += b;
      return f;
    }

    f.toString = function() {
      return currentSum;
    };

    return f;
  }

console.log( `sum = ${sum(1)(2)} `); 

//inheritance
let head = {
    glasses: 1
  };
  
  let table = {
    pen: 7,
    __proto__: head
  };
  
  let bed = {
    sheet: 1,
    pillow: 2,
    __proto__: table
  };
  
  let pockets = {
    money: 2000,
    __proto__: bed
  };


console.log(`pockets.glasses = ${pockets.glasses}`);

//f.prototype

function User(name) {
    this.name = name;
  }
//User.prototype = {};

let user = new User("vidhi");
let user2 = new user.constructor("bhayani");
console.log(`user2.name = ${user2.name}`);