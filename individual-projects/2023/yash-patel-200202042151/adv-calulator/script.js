
function fab(num) {
    if (num == 1) {
        return 1;
    }
    else {
        return num * fab(num - 1);
    }
}


function calculate() {
    let value = document.getElementById('display').value;
    if (value.includes("!")) {
        let Number = parseFloat(value.substring(0, value.indexOf('!')));
        let ans = fab(Number);
        document.getElementById('display').value = ans;
    }
    else if (value.includes("%")) {
        let firstvalue = parseFloat(value.substring(0, value.indexOf('%')));
        let lastvalue = parseFloat(value.substring(value.indexOf('%') + 1,));
        document.getElementById('display').value = firstvalue * lastvalue / 100;
    } else { 
    let userInput = value;
    let result = Function("return " + userInput)(); 
    document.querySelector("#display").value=result ;
    }
}


function backspace() {
    size = display.value.length;
    display.value = display.value.substring(0, size - 1);
}



function Sqrt() {
    let x = display.value;
    let ans = Math.sqrt(x);
    document.getElementById('display').value = ans;
}
