function remove() {
    document.forms["calculator"].elements['result'].value = "";
}

function del() {

    let a = document.forms["calculator"].elements['result'].value;
    document.forms["calculator"].elements['result'].value = a.substr(0, a.length - 1);
}

function number(value) {
    document.forms["calculator"].elements['result'].value += value;
}

function sqrt() {
    document.forms["calculator"].elements['result'].value = Math.pow(document.forms["calculator"].elements['result'].value, 1 / 2);
}

function square() {
    document.forms["calculator"].elements['result'].value = Math.pow(document.forms["calculator"].elements['result'].value, 2);
}

function fact() {
    let num = parseInt(document.forms["calculator"].elements['result'].value);
    let result = 1;
    for (let i = num; i > 0; i--) {
        result *= i;
    }
    document.forms["calculator"].elements['result'].value = result;
}

function equal() {
    document.forms["calculator"].elements['result'].value = eval(document.forms["calculator"].elements['result'].value);
}
function perc() {
    let num = parseInt(document.forms["calculator"].elements['result'].value);
    document.forms["calculator"].elements['result'].value = num / 100;
}