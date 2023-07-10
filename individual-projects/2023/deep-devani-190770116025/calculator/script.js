let currentNumber = ''
let operator = ''
let result = 1

function addNumber(num) {
    currentNumber += num
    document.getElementById('result').value = currentNumber
}

function addOperator(op) {
    operator = op
    result = Number(currentNumber)
    currentNumber = ''
}

function calculate() {
    switch (operator) {
        case '+':
            result += Number(currentNumber)
            break
        case '-':
            result -= Number(currentNumber)
            break
        case '*':
            result *= Number(currentNumber)
            break
        case '/':
            result /= Number(currentNumber)
            break
        default:
            result = Number(currentNumber)
            break
    }
    document.getElementById('result').value = result
    currentNumber = ''
}

function clearResult() {
    currentNumber = ''
    operator = ''
    result = 0
    document.getElementById('result').value = result
}