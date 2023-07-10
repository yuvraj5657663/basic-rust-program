let personAPoints = 0
let personBPoints = 0

function personA(a, b) {
    if (a == b) {
        personAPoints++
    }
}

function personB(x, y) {
    if (x == y) {
        personBPoints++
    }
}

function main() {
    let deck = [1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 0, 0]
    let totalCards = 20
    let totalRounds = 5

    personA(3, 4) // a0
    personB(3, 3) // b1

    personA(7, 8) // a0
    personB(4, 5) // b1

    personA(4, 4) // a1
    personB(5, 5) // b2

    personA(8, 8) // a2
    personB(6, 9) // b2

    personA(0, 9) //a2
    personB(6, 6) //b3

    console.log('personA points =', personAPoints)
    console.log('personB points =', personBPoints)

    if (personAPoints < personBPoints) {
        console.log('personB win')
    } 
    else if (personAPoints > personBPoints) {
        console.log('personA win')
    } 
    else {
        console.log('match draw')
    }
}
main()
