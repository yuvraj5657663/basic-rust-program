function calculator(...numbers){
    function sum(){
        let result = 0;
        for (let elements of numbers){
            result = result + elements;
        }
        return console.log(`sum=${result}`);
        
    }
    function sub(){
        result = numbers[0];
        for(let i = 1; i < numbers.length; i++){
            result = result - numbers[i];
        }
        return console.log(`sub=${result}`);
    }

    function mul(){
        let result = 1;
        for(let elements of numbers){
            result = result* elements;
        }
        return console.log(`mul=${result}`);
    }

    function div(){
        result = numbers[0];
        for (let i=1; i
        , numbers.length; i++){
            if( numbers[i] === 0 ){
                return console.log("Error: Division by zero is not allowed.");
            } else {
                result=result / numbers[i];
            }
            
            return console.log(`div=${result}`);
            
        }
            
        
    }
    
    return sum(),sub(),mul(),div();


}
calculator(2,1,1,1);

