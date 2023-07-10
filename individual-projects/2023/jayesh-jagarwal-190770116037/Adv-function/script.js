
//sum all number

function sumTo(n) {

    let sum = 0;

    for (let i = 1; i <= n;i++){

        sum = sum + i;
    }
    
    return console.log(`sum = ${sum}`);

}

sumTo(100);
sumTo(3);



//factorial

function factorial(n) {

    let factorial = 1;

    for (let i = 1; i <= n; i++){

        factorial = factorial * i;
    }
    return console.log(`factorial of 5 is = ${factorial}`);
}

factorial(5);


//Fibonacci

function fib(n) {
    let a = 1;
    let b = 1;
    for (let i = 3; i <= n; i++) {
      let c = a + b;
      a = b;
      b = c;
    }
    return b;
  }
  
  console.log(`fibonacci of 3 is = ${fib(3)}`); 
  console.log(`fibonacci of 7 is = ${fib(7)}`); 
  console.log(`fibonacci of 77 is = ${fib(77)}`); 


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


//Does a function pickup latest changes?

function makeWorker() {
  let name = "Jayesh";

  return function() {
    alert(name);
  };
}

let name = "John";

let work = makeWorker();

work();


//Function in if

let phrase = "Hello";

if (true) {
  let user = "Jayesh";

  function sayHi() {
    alert(`${phrase}, ${user}`);
  }
}

sayHi();


//Short by field

function byField(fieldName){
  return (a, b) => a[fieldName] > b[fieldName] ? 1 : -1;
}


//Set and decrease for counter

function makeCounter() {
  let count = 0;

  function counter() {
    return count++;
  }

  counter.set = value => count = value;

  counter.decrease = () => count--;

  return counter;
}


//Counter object

function Counter() {
    let count = 0;
  
    this.up = function() {
      return ++count;
    };
  
    this.down = function() {
      return --count;
    };
  }
  
  let counter = new Counter();
  
  console.log( counter.up() ); 
  console.log( counter.up() ); 
  console.log( counter.down() ); 


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

console.log( `sum of 5 and 6 is = ${sum(5)(6)} `); 

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


console.log(`pockets.money = ${pockets.money}`);


//f.prototype

function User(name) {
    this.name = name;
  }
//User.prototype = {};

let user = new User("John");
let user2 = new user.constructor("Jayesh");
console.log(`user2.name = ${user2.name}`);


//Output every second Using setInterval.

function printNumbers(from, to) {
  let current = from;

  let timerId = setInterval(function() {
    console.log(current);
    if (current == to) {
      clearInterval(timerId);
    }
    current++;
  }, 1000);
}

printNumbers(5,10);


//Output every second Using nested setTimeout.

function printNumbers(from, to) {
  let current = from;

  setTimeout(function go() {
    console.log(current);
    if (current < to) {
      setTimeout(go, 1000);
    }
    current++;
  }, 1000);
}

printNumbers(5, 10);

//Spy decorator

function spy(func) {

  function wrapper(...args) {

    wrapper.calls.push(args);
    return func.apply(this, args);
  }

  wrapper.calls = [];

  return wrapper;
}


//Bound function as a method

/* function f() {
  console.log( this ); // null
}

let user = {
  g: f.bind(null)
};

user.g(); */


//Second bind

function f() {
  console.log(this.name);
}

f = f.bind( {name: "Jay"} ).bind( {name: "Ann" } );

f();


//Working with prototype

let animal = {
  jumps: null
};
let rabbit = {
  __proto__: animal,
  jumps: true
};

console.log( rabbit.jumps ); // ? (1)

delete rabbit.jumps;

console.log( rabbit.jumps ); // ? (2)

delete animal.jumps;

console.log( rabbit.jumps ); // ? (3)


//Add method "f.defer(ms)" to functions

Function.prototype.defer = function(ms) {
  setTimeout(this, ms);
};

function f() {
  alert("Hello!");
}

f.defer(2000); // shows "Hello!" after 1 sec


