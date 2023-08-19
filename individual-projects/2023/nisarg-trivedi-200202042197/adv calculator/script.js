function button(a) {
    document.getElementById("display").value += a;
}
function buttonclear() {
    document.getElementById('display').value = " ";
}

function backspace() {
    size = display.value.length;
    display.value = display.value.substring(0, size - 1);
}


function cal() {
    let value = document.getElementById('display').value;

    if (value.includes("!")) {
        let Number = parseFloat(value.substring(0, value.indexOf('!')));
        let display = fab(Number);
        document.getElementById('display').value = display;
    }

    else if (value.includes("%")) {
        let firstvalue = parseFloat(value.substring(0, value.indexOf('%')));
        let lastvalue = parseFloat(value.substring(value.indexOf('%') + 1,));
        document.getElementById('display').value = firstvalue * lastvalue / 100;
    } 
    else { 
    let userInput = value;
    let result = Function("return " + userInput)(); 
    document.querySelector("#display").value=result ;
    }
}


function fab(num) {
    if (num == 1) {
        return 1;
    }
    else {
        return num * fab(num - 1);
    }
}

function Sqrt() {
    let A = display.value;
    let displays = Math.sqrt(A);
    document.getElementById('display').value = displays;
    
}
