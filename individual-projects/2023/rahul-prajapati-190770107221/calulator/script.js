calculator(50,2,5);

function calculator(...num){

    function Add(){
        let result = 0;
        for (let elements of num){
            result += elements;
        }
        return console.log("Addition of ("+num+") is =",result);
        
    }
    function Sub(){
        result = num[0];
        for(let i = 1; i < num.length; i++){
            result -= num[i];
        }
        return console.log("Subtraction of ("+num+") is =",result);
    }

    function Mul(){
        let result = 1;
        for(let elements of num){
            result *= elements;
        }
        return console.log("Multiplication of ("+num+") is =",result);
    }

    function Div(){
        result = num[0];
        for (let i=1; i< num.length; i++){
            if( num[i] === 0 ){
                return console.log("Error: Division by zero is not allowed.");
            } else {
                result /= num[i];
            }
        }return console.log("Division of ("+num+") is =",result);
    }
    return Add(),Sub(),Mul(),Div();
}