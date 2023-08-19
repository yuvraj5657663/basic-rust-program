
let pattern = "";
let raw = 5;


for (let i = 1; i <= raw; i++) {

    for (let space = i; space <= raw - 1; space++) {

        pattern += " ";
    }

    pattern += " *".repeat(i);

    

    pattern += "\n";
}
console.log(pattern);


