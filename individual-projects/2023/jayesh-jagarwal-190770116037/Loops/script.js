
let pattern1="";
for(let i=1;i<=4;i++){
    for(let j=0;j<i;j++){
        pattern1=pattern1+"*";    
    }
    pattern1=pattern1+"\n";
}

console.log(pattern1);

let pattern2="";

for(let i=1;i<=6;i++){

    for(let j=1;j<=6-i;j++){
        pattern2=pattern2+" ";
    }

    for(let k=0;k<i-1;k++){
        pattern2=pattern2+"* ";
    }
    pattern2=pattern2+"\n";
}

console.log(pattern2);


