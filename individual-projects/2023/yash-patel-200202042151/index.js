// <----name1 function ------>
function name1() {
    const fullname = prompt('enter the name', 'xyz')
    const age = +prompt('enter the age', '20')

    alert(age)
    alert(fullname)
}

name1()


// <----calculater function ------>

function calculater() {
    const a = +prompt('enter the number first')
    const b = +prompt('enter the number second')
    const operater = prompt('enter the operater')
    let ans

    if (a == null || b == null || operater == null) {
        alert('enter right value ')
        return
    }
    switch (operater) {
        case '+':
            ans = a + b
            alert(`the ans is  ${ans}`)
            break

        case '-':
            ans = a - b
            alert(`the ans is  ${ans}`)
            break

        case '*':
            ans = a * b
            alert(`the ans is  ${ans}`)
            break

        case '/':
            ans = a / b
            alert(`the ans is  ${ans}`)
            break

        default:
            alert('enter the defulat value')
            break
    }
}

calculater()


// <---- loop function ------>
function loop() {

    //   <--- for loop --->
    const n1 = 5
    for (let a = 0; a < n1; a++) {
        alert(a)
    }

    // <--- while loop ---> 
    let i = 0
    const n2 = 5
    while (i <= n2) {
        alert(i)
        i++
    }

    //  <--- do while --->
    let op = 1
    do {
        alert(op)
        op++
    } while (op <= 20)

    //  <--- for of --->
    const a = ['yash', 'xyz', 'abc']
    for (const iterator of a) {
        alert(iterator)
    }
}

loop()

//  <----arr function ------>

function arr() {
    const obj = ['yash', 'yash', 'xyz', 'abc', 'yash']
    let count = 0
    for (const iterator of obj) {
        if (iterator == 'yash') {
            count++
        }
    }
    alert(`the yash name count ${count} time `)
}

arr()
