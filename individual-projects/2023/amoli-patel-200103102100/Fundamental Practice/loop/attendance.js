//Take an attendance of class.
function main() {
    let total_student = 60
    for (number=1;number <= total_student;number++) {
        if (
            number === 2 ||
            number === 15 ||
            number === 23 ||
            number === 30 ||
            number === 42 ||
            number === 59
        ) {
            console.log(number,'absent')
            continue
        }
            console.log(number,'present')
    }
}
main()
