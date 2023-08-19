// start on/off
// temperature set
// according to cold / hot perform service
// if electricity goes then

function ac_start() {
    let temperature = prompt('Enter temperature:')
    let hot = 30,
        electricitiy
    if (temperature >= hot) {
        console.log('start the service')
        // service()
    } else {
        console.log('no service')
    }
    while (electricitiy == false) {
        console.log('no electricity')
        break
    }
}

function main() {
    let start_status
    if (start_status == true) {
        console.log('AC is turned ON')
        ac_start()
    } else {
        console.log('AC is OFF')
    }
}
main()
