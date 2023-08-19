// Sum of all Numbers

function sumOf(n) {
    if (n == 1)
  
      return 1;
  
    return n + sumOf(n - 1);
  }
  alert(sumOf(100));
  
  //factorial
  
  function fac(n) {
  
    let factorial = 1;
  
    for (let i = 1; i <= n; i++) {
  
      factorial = factorial * i;
    }
    return console.log(`factorial of 5 = ${factorial}`);
  }
  fac(5);
  
  //fibonaci serias
  
  const number = +(prompt("Enter the numbers : "));
  let a = 0; 
  let b = 1;
  let nextNum = 
  
  console.log('Fibonacci numbers:');
  for (let i = 1; i <= number; i++) {
      console.log(a);
      nextNum = a + b;
      a = b;
      b = nextNum;
  }
  
  //Output a single-linked list
  
  let fruits = {
    value: "orange",
    next: {
      value: "mango",
      next: {
        value: "apple",
        next: {
          value: "watermalen",
          next: null
        }
      }
    }
  };
  
  // Output a single-linked list in the order
  function printList(fruits) {
    console.log("printListss(fruits) = ");
    while (fruits) {
      console.log(fruits.value);
      fruits = fruits.next;
    }
  }
  printList(fruits);
  
  // variables and scopes examples 
  
  function userName()
   {
    let name = "Nisarg";
    return function() 
    {
      alert(name);
    };
  }
  let name = "Trivedi";
  let user1 = userName();
  user1();
  
  // Are counters independent
  
  function makeItems() {
    let i = 0;
  
    return function() 
    {
      return i++;
    };
  }
  
  let counter1 = makeItems();
  let counter2 = makeItems();
  
  console.log( counter1() );
  console.log( counter1() ); 
  
  console.log( counter2() ); 
  console.log( counter2() );
  
  //counter object
  
  const counter = {
    count: 0,
    up: function()
     {
      this.count++;
      console.log("Count up: " + this.count);
    },
    down: function()
     {
      this.count--;
      console.log("Count down: " + this.count);
    }
  };
  counter.up(); 
  counter.up(); 
  counter.down();
  
  // sum with closures
  
  function sum(num1) {
  
    return function(num2) 
    
    {
      return num1 + num2;
    };
  }
  alert( sum(7)(5) ); 
  alert( sum(5)(10) );
  
  // Sum with an arbitrary amount of brackets
  
  function sum(a) {
  
    let currSum = a;
    function f(b) {
      currSum += b;
      return f;
    }
  
    f.toString = function () {
      return currSum;
    };
    return f;
  }
  
  console.log(`sum = ${sum(3)(2)}`);
  
  
  //inheritance
  
  const showroom = {
    cars: 5
  };
  
  let basket = {
    fruit: 7,
    __proto__: showroom
  };
  
  let pockets = {
    money: 500,
    __proto__: basket
  };
  
  let tabel = {
    books: 2,
    __proto__: pockets
  };
  
  console.log(tabel.fruit);
  console.log(pockets.cars);
  
  
  // Native prototypes
  
  Function.prototype.defer = function (ms) 
  {
    setTimeout(this, ms);
  };
  
  function f() {
    alert("Welcome to Javascript");
  }
  
  f.defer(1000);
  
  
  // Settimeout task
  
  function printNumber(start, end) {
    let current = start;
  
    setTimeout(function next() {
      console.log(current);
      if (current < end) {
        setTimeout(next, 1000);
      }
      current++;
    }, 1000);
  }
  printNumber(1, 5);
  
  // prototype
  
  class animal {
    constructor(name, color) {
      this.name = name;
      this.color = color;
    }
  }
  let ani = new animal('Monkey', 'chimpu');
  let m = new animal("white", "black");
  
  console.log(ani.name);
  console.log(m.color);
  
  //f.prototype
  
  function User(name) {
    this.name = name;
  }
  
  let user = new User("Nisarg");
  let user2 = new user.constructor("Trivedi");
  console.log(`user2name = ${user2.name}`);
  
  