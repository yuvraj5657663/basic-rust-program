//-------------------------------Recursion and Stack--------------------------------


            //Task-1 Sum all numbers till the given one

                    // function sumTo(x){
                    //     let sum = 0;
                    //     for(i=0 ; i<=x ; i++){
                    //         sum = sum + i ;
                    //     }
                    //     return sum
                    // }
                    // console.log(sumTo(10));



            //sum using for Recursion

                    // function sumTo(n){
                    //     if(n==1){
                    //         return 1
                    //     }
                    //     return n + sumTo(n-1);
                    // }
                    // console.log(sumTo(10));



            //sum using for formula

                    // function sumTo(n){
                    //     return n*(n+1)/2;
                    // }
                    // console.log(sumTo(10));



                    
            //Task-2 factorial using Recursion

                    // function factorial(n) {
                    //     if (n == 0) {
                    //         return 1
                    //     }
                    //     return n * factorial(n - 1);
                    // }
                    // let data = 5;
                    // console.log(factorial(data));



            //Task-3 fibonacci using Recursion

                    //  function fibonacci(n) {
                    //     if (n <= 1) {
                    //             return n
                    //         }
                    //      return fibonacci(n-1) + fibonacci(n-2);
                    //      }
                    //     let data = 7;
                    //     console.log(fibonacci(data));

                

                    
            //Task-4 Single linked list

                //  let list = {
                //     value: 1, 
                //     next:{
                //         value: 2, 
                //         next:{ 
                //             value: 3, 
                //             next:{
                //                 value: 4, 
                //                 next: null
                //                  }
                //              }
                //          }
                //      };

                                //  function printlist(list){
                                //     console.log(list.value);
                                //     if(list.next){
                                //         printlist(list.next)
                                //     }
                                //  }
                                //  printlist(list);
                                
                                //// Reverse linked list using recursion
                                //  function printReverselist(list){
                                //     if(list.next){
                                //         printReverselist(list.next)
                                //     }
                                //     console.log(list.value);
                                //  }
                                //  printReverselist(list);


                                ////Reverse linked list using loop
                                //  function printReverselist(list){
                                //     let arr = [];
                                //     let temp = list;
                                //     while(temp){
                                //         arr.push(temp.value);
                                //         temp = temp.next;
                                //     }
                                //     for(i=arr.length-1; i>=0 ; i--){
                                //         console.log(arr[i]);
                                //     }
                                //  }
                                //  printReverselist(list);
                              
                                






//----------------------------------------Variable scope, closure ----------------------------------------




                                //Counter object
                                        // function Counter() {
                                        //         let count = 0;
                                        
                                        //         this.up = function() {
                                        //           return ++count;
                                        //         };
                                        
                                        //         this.down = function() {
                                        //           return --count;
                                        //         };
                                        //       }
                                        
                                        //       let counter = new Counter();
                                        
                                        //       console.log( counter.up() ); // 1
                                        //       console.log( counter.up() ); // 2
                                        //       console.log( counter.down() ); // 1


                                //Function in if
                                        // let phrase = "Hello";

                                        // if (true) {
                                        //   let user = "John";
                                        
                                        //   function sayHi() {
                                        //    console.log(`${phrase}, ${user}`);
                                        //   }
                                        // }
                                        
                                        // sayHi();

                                //Sum with closures
                                        //       function sum(a) {

                                        //         return function(b) {
                                        //           return a + b; // takes "a" from the outer lexical environment
                                        //         };
                                        
                                        //       }
                                        
                                        //       console.log( sum(1)(2) ); // 3
                                        //       console.log( sum(5)(-1) ); // 4

                                        







//----------------------------------------setTimeout & setInterval ----------------------------------------



        //Output every seconds


                //using setInterval

                        // function printNumbers(from,to){
                        //         let current = from;
                        //         let timerId = setInterval(function(){
                        //                 console.log(current);
                        //                 if(current == to){
                        //                         clearInterval(timerId);
                        //                 }
                        //                 current++;
                        //         },1000);
                        // }
                        // printNumbers(1,10);
                
                //using setTimeout

                        // function printNumbers(from,to){
                        //         let current = from;
                        //         setTimeout(function go(){
                        //                 console.log(current);
                        //                 if(current < to){
                        //                         setTimeout(go,1000);
                        //                 }
                        //                 current++;
                        //         },1000);
                        // }
                        // printNumbers(1,10);


                        //example
                                // let i = 0;
                                // setTimeout(()=> console.log(i),100);

                                // for(j=0; j<100000000; j++){
                                //         i++;
                                // }





 //--------------------------------- Prototypal, inheritance -------------------------------




                        //Task-1   Working with prototype

                                // let animal = {
                                //         jumps: null
                                //       };
                                //       let rabbit = {
                                //         __proto__: animal,
                                //         jumps: true
                                //       };
                                      
                                //       console.log( rabbit.jumps ); 
                                      
                                //       delete rabbit.jumps;
                                      
                                //       console.log( rabbit.jumps ); 
                                      
                                //       delete animal.jumps;
                                      
                                //       console.log( rabbit.jumps ); 


                        // Task-2  Searching algorithm

                                        // let head = {
                                        //         glasses: 1
                                        //       };
                                        
                                        //       let table = {
                                        //         pen: 3,
                                        //         __proto__: head
                                        //       };
                                        
                                        //       let bed = {
                                        //         sheet: 1,
                                        //         pillow: 2,
                                        //         __proto__: table
                                        //       };
                                        
                                        //       let pockets = {
                                        //         money: 2000,
                                        //         __proto__: bed
                                        //       };
                                        
                                        //       console.log( pockets.pen ); 
                                        //       console.log( bed.glasses ); 
                                        //       console.log( table.money ); 


                            // Task-3 
                                        // let hamster = {
                                        //         stomach: [],
                                        
                                        //         eat(food) {
                                        //         this.stomach.push(food);
                                        //         }
                                        // };
                                        
                                        // let speedy = {
                                        //         __proto__: hamster,
                                        //         stomach: []
                                        // };
                                        
                                        // let lazy = {
                                        //         __proto__: hamster,
                                        //         stomach: []
                                        // };
                                        
                                        // speedy.eat("apple");
                                        // console.log( speedy.stomach ); 
                                        
                                        // console.log( lazy.stomach ); 





                
                        // F.prototype

                                // Task--1  Changing "prototype"

                                    //ex-1
                                        // function Rabbit() {}
                                        // Rabbit.prototype = {
                                        //   eats: true
                                        // };
                                        
                                        // let rabbit = new Rabbit();
                                        
                                        // Rabbit.prototype = {};
                                        
                                        // console.log( rabbit.eats ); //true


                                   //ex-2
                                        // function Rabbit() {}
                                        // Rabbit.prototype = {
                                        //   eats: true
                                        // };
                                        
                                        // let rabbit = new Rabbit();
                                        
                                        // Rabbit.prototype.eats = false;
                                        
                                        // console.log( rabbit.eats ); // false

                                    //ex-3 
                                        // function Rabbit() {}
                                        // Rabbit.prototype = {
                                        //   eats: true
                                        // };
                                        
                                        // let rabbit = new Rabbit();
                                        
                                        // delete rabbit.eats;
                                        
                                        // console.log( rabbit.eats ); // true


                                    //ex-4  
                                        // function Rabbit() {}
                                        // Rabbit.prototype = {
                                        //   eats: true
                                        // };
                                        
                                        // let rabbit = new Rabbit();
                                        
                                        // delete Rabbit.prototype.eats;
                                        
                                        // console.log( rabbit.eats ); // undefined

                                        
                            // Task--2   Create an object with the same constructor  

                                    //ex-1      
                                        //     function User(name) {
                                        //         this.name = name;
                                        //       }
                                              
                                        //       let user = new User('John');
                                        //       let user2 = new user.constructor('Pete');
                                              
                                        //       console.log( user2.name ); // Pete

                                   //ex-2
                                        //      function User(name) {
                                        //                 this.name = name;
                                        //         }
                                        //         User.prototype = {}; // (*)
                                                
                                        //         let user = new User('John');
                                        //         let user2 = new user.constructor('Pete');
                                                
                                        //         console.log( user2.name ); // undefined






                                        
                // Native prototypes

                        //Task-1  Add method "f.defer(ms)" to functions
                                        // Function.prototype.defer = function(ms) {
                                        //         setTimeout(this, ms);
                                        //       };
                                              
                                        //       function f() {
                                        //         console.log("Hello!");
                                        //       }
                                              
                                        //       f.defer(1000); // shows "Hello!" after 1 sec


                        //Task-2 Add the decorating "defer()" to functions
                                        //       Function.prototype.defer = function(ms) {
                                        //         let f = this;
                                        //         return function(...args) {
                                        //           setTimeout(() => f.apply(this, args), ms);
                                        //         }
                                        //       };
                                              
                                        //       let user = {
                                        //         name: "John",
                                        //         sayHi() {
                                        //          console.log(this.name);
                                        //         }
                                        //       }
                                              
                                        //       user.sayHi = user.sayHi.defer(1000);
                                              
                                        //       user.sayHi();







                        //Prototype methods, objects without __proto__


                                    //Add toString to the dictionary
                                            // let dictionary = Object.create(null, {
                                            //         toString: { // define toString property
                                            //           value() { // the value is a function
                                            //             return Object.keys(this).join();
                                            //           }
                                            //         }
                                            //       });
                                            
                                            //       dictionary.apple = "Apple";
                                            //       dictionary.__proto__ = "test";
                                            
                                            //       for(let key in dictionary) {
                                            //         console.log(key); // "apple", then "__proto__"
                                            //       }
                                    
                                            //       console.log(dictionary); // "apple,__proto__"



                                    //The difference between calls
                                            //  function Rabbit(name) {
                                            //    this.name = name;
                                            //   }
                                            //   Rabbit.prototype.sayHi = function() {
                                            //     console.log( this.name );
                                            //   }
                                                        
                                            //    let rabbit = new Rabbit("Rabbit");
                                                        
                                            //   rabbit.sayHi();                        // Rabbit
                                            //   Rabbit.prototype.sayHi();              // undefined
                                            //   Object.getPrototypeOf(rabbit).sayHi(); // undefined
                                            //   rabbit.__proto__.sayHi();              // undefined