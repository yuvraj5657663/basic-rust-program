console.log('PATTERN 1')

for (let i = 1; i <= 5; i++) {
    let line = ''
    for (let j = 1; j <= i; j++) {
        line += '*'
    }
    console.log(line)
}

console.log('PATTERN 2')
const numRows = 5
for (let i = 0; i < numRows; i++) {
    let row = ''
    for (let j = 0; j < numRows - i - 1; j++) {
        row += ' '
    }
    for (let j = 0; j <= i; j++) {
        row += '* '
    }
    console.log(row)
}
