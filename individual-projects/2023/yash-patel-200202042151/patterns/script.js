// loop

let pattern = ''
for (let i = 0; i < 5; i++) {
    for (let j = 0; j < i; j++) {
        pattern += '* '
    }
    pattern += '\n'
}

console.log(pattern)

let pattern1 = ''
const n = 5

for (let i = 1; i <= n; i++) {
    for (let j = 1; j <= n - i; j++) {
        pattern1 += ' '
    }
    for (let a = 1; a <= i - 1; a++) {
        pattern1 += '* '
    }
    pattern1 += '\n'
}

console.log(pattern1)
