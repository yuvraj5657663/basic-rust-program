// Sum all numbers till the given one

function sumTO(n) {
    if (n == 1) {
        return 1;
    } else {
        return n + sumTO(n - 1);
    }
}
alert(sumTO(5));

// Calculate factorial

function factorial(n) {
    if (n == 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
alert(factorial(5));

// Fibonacci numbers

function fib(n) {
    if (n <= 1) {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
alert(fib(3));

// single-linked list

let list = {
    value: "yash",
    next: {
        value: "feneel",
        next: {
            value: "nisarg",
            next: {
                value: "friend",
                next: null,
            },
        },
    },
};
// Output a single-linked list in the order
function listnode(list) {
    alert(list.value);
    if (list.next) {
        listnode(list.next);
    }
}
listnode(list);

// Output a single-linked list in the reverse order

function listnode_reverse(list) {
    if (list.next) {
        listnode_reverse(list.next);
    }
    alert(list.value);
}
listnode_reverse(list);

// Output every second

function printNumbers(start, end) {
    let current = start;

    setTimeout(function next() {
        console.log(current);
        if (current < end) {
            setTimeout(next, 1000);
        }
        current++;
    }, 1000);
}
printNumbers(5, 10);

// inheritance

const head = {
    glasses: 1
};

const table = {
    pen: 3,
    __proto__: head
};

const bed = {
    sheet: 1,
    pillow: 2,
    __proto__: table
};

const pockets = {
    money: 2000,
    __proto__: bed
};
alert(pockets.pen);
alert(bed.glasses);

// prototype

class employe {
    constructor(surname, name, age, work) {
        this.surname = surname;
        this.name = name;
        this.age = age;
        this.work = work;
    }
}

let yash = new employe('Patel', 'yash', 18, 'javascript');
let user2 = new yash.constructor('Patel', 'xyz', 180, 'java');

alert(yash.age);
alert(`user2.name = ${user2.name}`);
