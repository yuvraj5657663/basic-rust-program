const n = 5;
// Take variable ( pattern1 ) is empty string which is used for pattern 1
let pattern1 = "";
for(let i = 1; i<=n; i++){
    for(let j = 0; j < i; j++){
        pattern1 = pattern1+ "*";
    }
    pattern1 =pattern1+ "\n";
}

console.log( pattern1 );

let pattern2 = "";

for(let i = 1; i <= n; i++){

    for(let j = 1; j <= n-i; j++){
        pattern2 = pattern2 +" ";
    }

    for(let k= 0; k<i-1; k++){
        pattern2 = pattern2 +"* ";
    }
    pattern2 = pattern2 +"\n";
}

console.log( pattern2 );