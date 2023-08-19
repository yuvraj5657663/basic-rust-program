function calculate() {
    let value = document.getElementById('ans').value;
    if (value.includes("!")) {
        let Number = parseFloat(value.substring(0, value.indexOf('!')));
        let ans = fab(Number);

        document.getElementById('ans').value = ans;
    }
    else if (value.includes("%")) {
        let firstvalue = parseFloat(value.substring(0, value.indexOf('%')));
        let lastvalue = parseFloat(value.substring(value.indexOf('%') + 1,));

        document.getElementById('ans').value = firstvalue * lastvalue / 100;
    } else {
        let userInput = value;
        let result = Function("return " + userInput)();

        document.querySelector("#ans").value = result;
    }
}

document.getElementById("ans").addEventListener("keypress", onkeypressAnswer);
function onkeypressAnswer(event) {
    if (event.keyCode === 13) {
        calculate();
    }
}

function button(a) {
    document.getElementById("ans").value += a;
}

function buttonclear() {
    document.getElementById('ans').value = " ";
}

function Sqrt() {
    let x = ans.value;
    let anss = Math.sqrt(x);
    document.getElementById('ans').value = anss;
}

function fab(num) {
    if (num == 1) {
        return 1;
    }
    else {
        return num * fab(num - 1);
    }
}