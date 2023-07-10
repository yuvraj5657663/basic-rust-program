//Pattern 1

let pat1 = "";
for (i=1; i<=5; i++) {

    for (j=1; j<=i; j++) {
    pat1=pat1+"*";
    }

    pat1=pat1+"\n";
}
console.log(pat1);



//Pattern 2
let num= 5;
let pat2 = "";

for(let i=1; i<=num; i++){
    for(let j=1; j<=num-i; j++){
        pat2 += " ";
    }

    for(let k=1; k<=2*i-1; k++){
        pat2 += "*";
    }
    
    pat2 += "\n";
}
console.log(pat2)