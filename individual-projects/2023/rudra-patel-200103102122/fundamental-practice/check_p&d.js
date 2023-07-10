// ====================== switch case ======================
// petrol & diesel of my vehicle

function petrol_diesel() {
    let full_tank_p_d = 5
    if (full_tank_p_d == 1) {
        console.log('the petrol or deisel is 1L you have to fill the tank')
        full_tank_p_d++
    } else if (full_tank_p_d < 5) {
        console.log('the petrol or diesel is less than 5L')
        full_tank_p_d++
    } else if (full_tank_p_d == 5) {
        console.log('the petrol or diesel is full')
    } else {
        console.log('the petrol or diesel is empty')
    }
}
function main() {
    let level = 5
    switch (level) {
        case '5':
            console.log('the petrol or diesel is full')
            break
        case '4':
            console.log('the petrol or diesel is 4L')
            petrol_diesel()
            break
        case '3':
            console.log('the petrol or diesel is 3L')
            petrol_diesel()
            break
        case '2':
            console.log('the petrol or diesel is 2L')
            petrol_diesel()
            break
        case '1':
            console.log('the petrol or diesel is 1L')
            petrol_diesel()
            break
        default:
            console.log('the petrol or diesel is empty')
            petrol_diesel()
            break
    }
}
