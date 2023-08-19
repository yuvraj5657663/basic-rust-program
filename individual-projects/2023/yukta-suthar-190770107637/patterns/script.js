let a="";
for(let i=1;i<=4;i++){
    for(let j=0;j<i;j++){
        a=a+"*";
    }
    a=a+"\n";
}

console.log(a);

let b="";

for(let i=1;i<=5;i++){

    for(let j=1;j<=5-i;j++){
        b=b+" ";
    }

    for(let k=0;k<i-1;k++){
        b=b+"* ";
    }
    b=b+"\n";
}

console.log(b);
