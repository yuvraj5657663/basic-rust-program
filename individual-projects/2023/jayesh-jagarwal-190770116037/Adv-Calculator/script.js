function factorial(num) {
    if (num === 0 || num === 1) {
        return 1;
    }
    else {
        return num * factorial(num - 1);
    }
}
function getPercentage() {
    let input = document.getElementById('result').value
    let result = input / 100
    document.getElementById('result').value = result
}

function getSqrt() {
    let input = document.getElementById("result");
    input.value = Math.sqrt(parseFloat(input.value));
}


function calculate() {
    let input = document.getElementById('result').value
    if (input) {
        try {
            var sum = new Function('return ' + input)
            var result = sum()
            document.getElementById('result').value = result
        } catch (e) {
            alert('Invalid Input')
        }
    }

}
