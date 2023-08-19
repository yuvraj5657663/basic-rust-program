//sum all number Task

function sumAll(n) {
    let sum = 0;
    
        for (let i = 1; i <= n;i++){
    
            sum = sum + i;
        }
    return console.log(`sum of all = ${sum}`);
    
    }
    sumAll(500);


// Factorial task
    
 function factorial(n) {
        if (n == 1) {
            return 1;
        } else {
            return n * factorial(n - 1);
        }
    }
    console.log(factorial(10));
    
 // fibonacci task
    
    const number = +(prompt("Enter the number: "));
    let num1 = 0; 
    let num2 = 1;
    let nextNum = 
    
    console.log('Fibonacci numbers:');
    
    for (let i = 1; i <= number; i++) {
        console.log(num1);
        nextNum = num1 + num2;
        num1 = num2;
        num2 = nextNum;
    }
    
 //Output a single-linked list task
    
    let list = {
        value: "feneel",
        next: {
          value: "yash",
          next: {
            value: "nisarg",
            next: {
              value: "jainam",
              next: null
            }
          }
        }
      };
    function outputSingleList(list) {
        while (list) {
          console.log(list.value);
          list = list.next;
        }
      }
    
    outputSingleList(list);
    
    //Output a single-linked list in reverse order task
    
    let list2 = {
        value: "hi",
        next: {
          value: "hey",
          next: {
            value: "hello",
            next: {
              value: "bye",
              next: null
            }
          }
        }
      };
      
      function outputReverseList(list) {
        let arr = [];
        let tmp = list;
      
        while (tmp) {
          arr.push(tmp.value);
          tmp = tmp.next;
        }
      
        for (let i = arr.length - 1; i >= 0; i--) {
          alert( arr[i] );
        }
      }
       outputReverseList(list);
       
       // sum with closure task
       
    function sum(num1) {
    
        return function(num2) {
          return num1 + num2; 
        };
    
      }
      console.log(sum(4)(6)); 
      
      // counter Object task
    
    function measure() {
      let count = 0
    
      this.up = function () {
          return ++count
      }
      this.down = function () {
          return --count
      }
    }
    
    let counter = new measure()
    console.log(counter.up())
    console.log(counter.down())
    
    // filter through function task

    function inbox(arr) {
        return function(x) {
          return arr.includes(x);
        };
      }
      
      let arr = [1, 2, 3, 4, 5, 6, 7];
      alert( arr.filter(inbox([1, 2, 10])) );
      
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
    printNumber(1, 10);
    
    
    // Nested settimeout task
    
    function numbers(from, to) {
      let current = from;
    
      setTimeout(function go() {
        console.log(current);
        if (current < to) {
          setTimeout(go, 2000);
        }
        current++;
      }, 2000);
    }
    numbers(1, 10);
    
 // Fix a function that loses "this" task
    
    function askPassword(ok, fail) {
        let password = prompt("Password?", '');
        if (password == "feneel") ok();
        else fail();
      }
      
      let user = {
        name: 'feneel',
      
        loginOk() {
          alert(`${this.name} logged in`);
        },
      
        loginFail() {
          alert(`${this.name} failed to log in`);
        },
      
      };
      
      askPassword(user.loginOk, user.loginFail);
      
      // F-prototype(create an object with same costructor)
    
    function User1(name) {
      this.name = name;
    }
    
    let user = new User1("feneel");
    let user2 = new user.constructor("yash");
    
    alert( user.name );
    alert(user2.name);
    
    // prototype task
    
    class employee {
        constructor(name, age, work) {
            this.name = name;
            this.age = age;
            this.work = work;
        }
    }
    
    let feneel = new employee('feneel', 20, 'Backend-devloper');
    let yash = new feneel.constructor('yash', 18, 'Backend-devloper');
    
    console.log(feneel.age);
    console.log(`user2 name = ${yash.name}`);
    
     // inheritance task
    
    const face = {
        nose: 1
    };
    
    const bag = {
        pen: 3,
        __proto__: face
    };
    
    const drawer = {
        book: 1,
        eraser: 2,
        __proto__: bag
    };
    
    const fridge = {
        apple: 2,
        __proto__: drawer
    };
    console.log(fridge.pen);
    console.log(drawer.nose);
    
    // sum with closure task
    function sum(num1) {
    
        return function(num2) {
          return num1 + num2; 
        };
    
      }
      console.log(sum(4)(6));
      
      // toString to dictionary task

let dictionary = Object.create(null, {
    toString: {value() {
        return Object.keys(this).join();
      }
    }
  });
  dictionary.name = "feneel";
  dictionary.surname = "solanki";
  for(let key in dictionary) {
    console.log(key);
  }
  console.log(dictionary); 


// f.defers method task

Function.prototype.defer = function(ms) {
    setTimeout(this, ms);
  };
  
  function feneel() {
    alert("Hello from feneel");
  }
  
  feneel.defer(1000); 


//decorating defers task

Function.prototype.defer = function(ms) {
    let name = this;
    return function(...args) {
      setTimeout(() => name.apply(this, args), ms);
    }
  };
  
  
  function fen(a, b) {
    alert( a + b );
  }
  
  fen.defer(2000)('hey', ' feneel');

// searching algorithm task

  let head = {
    glasses: 1
  };
  
  let table = {
    pen: 3,
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
  
  alert( pockets.pen );
  alert( bed.glasses );
  alert( table.money );
