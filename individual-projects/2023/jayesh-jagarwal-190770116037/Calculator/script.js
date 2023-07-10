function add(number){
    return result = result + number;    
}

function sub(number){
    return result = result - number;
}

function multiply(number){
    return result = result * number;
}

function divide(number){
    if(number === 0){
        return console.log("Error: Division by zero is not allowed.");
    } else {
        return result = result / number;
    }
}

let result = 0;

add(7)
add(7)


console.log(`result =   ${result}`);