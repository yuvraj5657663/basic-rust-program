function calculater(num1, operator, num2){
    if(operator === "+"){
        return num1 + num2 
    }else if(operator === "-"){
        return num1 - num2
    }else if(operator === "*"){
        return num1 * num2;
    }else if(operator === "/"){
        return num1 / num2;
    }else{
        return 'invalid operator'
    } 
     return num1(), operatore(), num2();
}

console.log("+", calculater(10, "+", 5))
console.log("-", calculater(10, "-", 5))
console.log("*", calculater(10, "*", 5))
console.log("/", calculater(10, "/", 5))
console.log("error",calculater(10, "a", 5))