// Assume 14 floor building, and Today Lift is not working and you are in building to repair it. Go through stairs and check where is the lift, repair it and come back in lift.

function main() {
    let person = 0,
        lift = 4,
        total_floors = 14
    for (check = 0; check <= total_floors; check++) {
        if (lift === person) {
            console.log(check, 'floor.....Got lift now repair')
            for (lift = check; lift >= 0; lift--) {
                console.log(lift, 'floor.....Coming down')
            }
            break
        } else {
            console.log(check, 'floor.....Going up')
            person++
        }
    }
}
main()
