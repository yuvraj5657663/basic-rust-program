function buttonPressed(button) {
    document.getElementById('input').value += button
}

function buttonSqrt() {
    let input = document.getElementById('input').value
    let result = Math.sqrt(input)
    document.getElementById('input').value = result
}

function buttonFactorial() {
    let input = document.getElementById('input').value
    let result = 1
    for (let i = 2; i <= input; i++) {
        result *= i
    }
    document.getElementById('input').value = result
}

function buttonPercentage() {
    let input = document.getElementById('input').value
    let result = input / 100
    document.getElementById('input').value = result
}
function calculate() {
    var input = document.getElementById('input').value
    if (input) {
        try {
            var sum = new Function('return ' + input)
            var result = sum()
            document.getElementById('input').value = result
        } catch (e) {
            alert('Invalid Input')
        }
    }
}

function clearInput() {
    document.getElementById('input').value = ''
}
