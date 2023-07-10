// loop

// pattern1
const n = 5;
let pattern1 = " ";
for (let i = 0; i < n; i++) {
  for (let j = 0; j < i; j++) {
    pattern1 += "* ";
  }
  pattern1 += "\n";
}
console.log(pattern1);

// pattern 2

const m = 6;
let pattern2 = "";
for (let i = 1; i <= m; i++) {
  for (let j = 1; j <= m - i; j++) {
    pattern2 += " ";
  }
  for (let k = 1; k <= i - 1; k++) {
    pattern2 += " *";
  }
  pattern2 += "\n";
}

console.log(pattern2);
