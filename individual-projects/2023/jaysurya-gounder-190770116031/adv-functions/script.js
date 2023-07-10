//  Recursion and stack
//  Write a function sumTo(n) that calculates the sum of numbers 1 + 2 + ... + n
//  1. Using a for loop.

function sumTo(n) {
    let sum = 0
    for (let i = 1; i <= n; i++) {
        sum += i
    }
    return sum
}
console.log(sumTo(100))

// 2.Using a recursion, cause sumTo(n) = n + sumTo(n-1) for n > 1.

function sumTo(n) {
    if (n === 1) {
        return 1
    } else {
        return n + sumTo(n - 1)
    }
}
console.log(sumTo(5))

//  3.Using the arithmetic progression formula.

function sumTo(n) {
    return (n * (n + 1)) / 2
}
console.log(sumTo(5))

//  Write a function factorial(n) that calculates n! using recursive calls.

function factorial(n) {
    if (n === 0 || n === 1) {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}
console.log(factorial(5))

// Write a function fib(n) that returns the n-th Fibonacci number.

function fib(n) {
    if (n < 2) {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}
console.log(fib(3))

// Variable scope, closure.
// Does a function pickup latest changes

let name = 'John'

function sayHi() {
    alert('Hi, ' + name)
}

name = 'Pete'

sayHi() // "Pete"

//  Which variables are available

function makeWorker() {
    let name = 'Pete'

    return function () {
        alert(name)
    }
}

let name = 'John'

let work = makeWorker()
work() // "Pete"

// Write a function printList(list) that outputs list items one-by-one.

function printList(list) {
    for (let i = 0; i < list.length; i++) {
        console.log(list[i])
    }
}
let myList = ['apple', 'banana', 'cherry']
printList(myList)

// Are counters independent

function makeCounter() {
    let count = 0

    return function () {
        return count++
    }
}

let counter = makeCounter()
let counter2 = makeCounter()

alert(counter()) // 0
alert(counter()) // 1

alert(counter2()) // 0
alert(counter2()) // 1

// Function in if

let phrase = 'Hello'

if (true) {
    let user = 'John'

    function sayHi() {
        alert(`${phrase}, ${user}`)
    }
}

sayHi() // error

// Write a function printList(list) that outputs list items one-by-one in reverse order.

function printList(list) {
    for (let i = list.length - 1; i >= 0; i--) {
        console.log(list[i])
    }
}
let myRerverseList = ['apple', 'banana', 'cherry']
printList(myRerverseList)

//Write function sum that works like this: sum(a)(b) = a+b.

function sum(a) {
    return function (b) {
        return a + b
    }
}
console.log(sum(2)(3))

//counter object

function Counter() {
    let count = 0

    this.up = function () {
        return ++count
    }
    this.down = function () {
        return --count
    }
}

let counter = new Counter()
alert(counter.up()) // 1
alert(counter.up()) // 2
alert(counter.down()) // 1

// Is variable visible?

let x = 1

function func() {
    console.log(x) // ERROR
    let x = 2
}

func()

// Filter inBetween

function inBetween(a, b) {
    return function (x) {
        return x >= a && x <= b
    }
}

let arr = [1, 2, 3, 4, 5, 6, 7]
console.log(arr.filter(inBetween(3, 6)))

// Army of functions

function makeArmy() {
    let shooters = []

    for (let i = 0; i < 10; i++) {
        let shooter = function () {
            // shooter function
            alert(i) // should show its number
        }
        shooters.push(shooter)
    }

    return shooters
}

let army = makeArmy()

army[0]() // 0
army[5]() // 5

//   Sort by field

let users = [
    { name: 'John', age: 20, surname: 'Johnson' },
    { name: 'Pete', age: 18, surname: 'Peterson' },
    { name: 'Ann', age: 19, surname: 'Hathaway' },
]
// by name (Ann, John, Pete)
users.sort((a, b) => (a.name > b.name ? 1 : -1))

// by age (Pete, Ann, John)
users.sort((a, b) => (a.age > b.age ? 1 : -1))

users.sort(byField('name'))
users.sort(byField('age'))

function byField(fieldName) {
    return (a, b) => (a[fieldName] > b[fieldName] ? 1 : -1)
}

// Function object, NFE
// Set and decrease for counter

function makeCounter() {
    let count = 0

    function counter() {
        return count++
    }

    counter.set = (value) => (count = value)

    counter.decrease = () => count--

    return counter
}

//  Is variable visible

let x = 1

function func() {
    console.log(x)
    let x = 2
}

func()

function func() {
    console.log(x)

    let x = 2
}

// Set and decrease for counter

function makeCounter() {
    let count = 0

    function counter() {
        return count++
    }

    counter.set = (value) => (count = value)

    counter.decrease = () => count--

    return counter
}

//  Sum with an arbitrary amount of brackets

function sum(a) {
    let currentSum = a

    function f(b) {
        currentSum += b
        return f
    }

    f.toString = function () {
        return currentSum
    }

    return f
}

alert(sum(1)(2)) // 3
alert(sum(5)(-1)(2)) // 6
alert(sum(6)(-1)(-2)(-3)) // 0
alert(sum(0)(1)(2)(3)(4)(5)) // 15

// Scheduling: setTimeout and setInterval
// Write a function printNumbers(from, to) that outputs a number every second, starting from from and ending with to.
// 1. Using setInterval.

function printNumbers(from, to) {
    let current = from

    let timerId = setInterval(function () {
        console.log(current)
        if (current == to) {
            clearInterval(timerId)
        }
        current++
    }, 1000)
}
printNumbers(5, 10)

// 2. Using nested setTimeout:

function printNumbers(from, to) {
    let current = from

    setTimeout(function go() {
        console.log(current)
        if (current < to) {
            setTimeout(go, 1000)
        }
        current++
    }, 1000)
}
printNumbers(5, 10)

// setTimeout call scheduled

let i = 0

setTimeout(() => console.log(i), 100) // 100000000

for (let j = 0; j < 100000000; j++) {
    i++
}

// Function binding
// Bound function as a method

function f() {
    alert(this) // null
}

let user = {
    g: f.bind(null),
}

user.g()

//  Second bind

function f() {
    alert(this.name)
}

f = f.bind({ name: 'John' }).bind({ name: 'Pete' })

f() // John

// Function property after bind

function sayHi() {
    alert(this.name)
}
sayHi.test = 5

let bound = sayHi.bind({
    name: 'John',
})

alert(bound.test) //  undefined

// Fix a function that loses "this"

function askPassword(ok, fail) {
    let password = prompt('Password?', '')
    if (password == 'rockstar') ok()
    else fail()
}

let user = {
    name: 'John',

    loginOk() {
        alert(`${this.name} logged in`)
    },

    loginFail() {
        alert(`${this.name} failed to log in`)
    },
}

askPassword(user.loginOk.bind(user), user.loginFail.bind(user))

// Partial application for login

function askPassword(ok, fail) {
    let password = prompt('Password?', '')
    if (password == 'rockstar') ok()
    else fail()
}

let user = {
    name: 'John',

    login(result) {
        alert(this.name + (result ? ' logged in' : ' failed to log in'))
    },
}

askPassword(user.login.bind(user, true), user.login.bind(user, false))

// Decorators and forwarding, call/apply
// Create a decorator spy(func) that should return a wrapper that saves all calls to function in its calls property.

function work(a, b) {
    alert(a + b) // work is an arbitrary function or method
}

work = spy(work)

work(1, 2) // 3
work(4, 5) // 9

for (let args of work.calls) {
    alert('call:' + args.join()) // "call:1,2", "call:4,5"
}
function spy(func) {
    function wrapper(...args) {
        // using ...args instead of arguments to store "real" array in wrapper.calls
        wrapper.calls.push(args)
        return func.apply(this, args)
    }

    wrapper.calls = []

    return wrapper
}

// Create a decorator delay(f, ms) that delays each call of f by ms milliseconds.

function delay(f, ms) {
    return function () {
        setTimeout(() => f.apply(this, arguments), ms)
    }
}

let f1000 = delay(alert, 1000)

f1000('test')

// Debounce decorator

let f = _.debounce(alert, 1000)

f('a')
setTimeout(() => f('b'), 200)
setTimeout(() => f('c'), 500) // debounced function waits 1000ms after the last call and then runs: alert("c")

function debounce(func, ms) {
    let timeout
    return function () {
        clearTimeout(timeout)
        timeout = setTimeout(() => func.apply(this, arguments), ms)
    }
}

// Create a “throttling” decorator throttle(f, ms) – that returns a wrapper.

function throttle(func, ms) {
    let isThrottled = false,
        savedArgs,
        savedThis

    function wrapper() {
        if (isThrottled) {
            savedArgs = arguments
            savedThis = this
            return
        }
        isThrottled = true

        func.apply(this, arguments) // (1)

        setTimeout(function () {
            isThrottled = false // (3)
            if (savedArgs) {
                wrapper.apply(savedThis, savedArgs)
                savedArgs = savedThis = null
            }
        }, ms)
    }

    return wrapper
}

//Prototypal inheritance
// Here’s the code that creates a pair of objects, then modifies them.

let animal = {
    jumps: null,
}
let rabbit = {
    __proto__: animal,
    jumps: true,
}

alert(rabbit.jumps) // true

delete rabbit.jumps

alert(rabbit.jumps) // null

delete animal.jumps

alert(rabbit.jumps) // undefined

// Searching algorithm

let head = {
    glasses: 1,
}

let table = {
    pen: 3,
    __proto__: head,
}

let bed = {
    sheet: 1,
    pillow: 2,
    __proto__: table,
}

let pockets = {
    money: 2000,
    __proto__: bed,
}

alert(pockets.pen)
alert(bed.glasses)
alert(table.money)

// If we call rabbit.eat(), which object receives the full property: animal or rabbit

let animal = {
    eat() {
        this.full = true
    },
}

let rabbit = {
    __proto__: animal,
}

rabbit.eat()

// We have two hamsters: speedy and lazy inheriting from the general hamster object.

let hamster = {
    stomach: [],

    eat(food) {
        this.stomach.push(food)
    },
}

let speedy = {
    __proto__: hamster,
    stomach: [],
}

let lazy = {
    __proto__: hamster,
    stomach: [],
}

speedy.eat('apple')
alert(speedy.stomach)
alert(lazy.stomach)

// F.prototype
//In the code below we create new Rabbit, and then try to modify its prototype.

function Rabbit() {}
Rabbit.prototype = {
    eats: true,
}

let rabbit = new Rabbit()

alert(rabbit.eats) // true

// We added one more string (emphasized). What will alert show now?

function Rabbit() {}
Rabbit.prototype = {
    eats: true,
}

let rabbit = new Rabbit()

Rabbit.prototype = {}

alert(rabbit.eats) // true

// …And if the code is like this (replaced one line)?

function Rabbit() {}
Rabbit.prototype = {
    eats: true,
}

let rabbit = new Rabbit()

Rabbit.prototype.eats = false

alert(rabbit.eats) // false

// And like this (replaced one line)?

function Rabbit() {}
Rabbit.prototype = {
    eats: true,
}

let rabbit = new Rabbit()

delete rabbit.eats

alert(rabbit.eats) // true

// The last variant:

function Rabbit() {}
Rabbit.prototype = {
    eats: true,
}

let rabbit = new Rabbit()

delete Rabbit.prototype.eats

alert(rabbit.eats) // undefined

// Create an object with the same constructor

function User(name) {
    this.name = name
}

let user = new User('John')
let user2 = new user.constructor('Pete')

alert(user2.name) // Pete (worked!)

function User(name) {
    this.name = name
}
User.prototype = {} // (*)

let user = new User('John')
let user2 = new user.constructor('Pete')

alert(user2.name) // undefined

// Native prototypes
//Add method "f.defer(ms)" to functions

Function.prototype.defer = function (ms) {
    setTimeout(this, ms)
}

function f() {
    alert('Hello!')
}

f.defer(1000) // shows "Hello!" after 1 sec

// Add the decorating "defer()" to functions

Function.prototype.defer = function (ms) {
    let f = this
    return function (...args) {
        setTimeout(() => f.apply(this, args), ms)
    }
}

// check it
function f(a, b) {
    alert(a + b)
}

f.defer(1000)(1, 2) // shows 3 after 1 sec

Function.prototype.defer = function (ms) {
    let f = this
    return function (...args) {
        setTimeout(() => f.apply(this, args), ms)
    }
}

let user = {
    name: 'John',
    sayHi() {
        alert(this.name)
    },
}

user.sayHi = user.sayHi.defer(1000)

user.sayHi()

// The JavaScript languagePrototypes, inheritance
// Add toString to the dictionary

let dictionary = Object.create(null, {
    toString: {
        // define toString property
        value() {
            // the value is a function
            return Object.keys(this).join()
        },
    },
})

dictionary.apple = 'Apple'
dictionary.__proto__ = 'test'

// apple and __proto__ is in the loop
for (let key in dictionary) {
    alert(key) // "apple", then "__proto__"
}

// comma-separated list of properties by toString
alert(dictionary) // "apple,__proto__"

// The difference between calls

function Rabbit(name) {
    this.name = name
}
Rabbit.prototype.sayHi = function () {
    alert(this.name)
}

let rabbit = new Rabbit('Rabbit')

rabbit.sayHi() // Rabbit
Rabbit.prototype.sayHi() // undefined
Object.getPrototypeOf(rabbit).sayHi() // undefined
