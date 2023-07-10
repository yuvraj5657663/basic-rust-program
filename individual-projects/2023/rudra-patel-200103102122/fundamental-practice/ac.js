let temperature = 25

function service(temperature) {
    let room_temp = 25

    if (room_temp == temperature) {
        console.log('no need Ac service')
    } else {
        console.log('plz do Ac service')
    }
}

function electricity() {
    let is_electricity = true

    if (is_electricity) {
        console.log('you have to turn on generator')
    }
    else {
        console.log('wait for elecricity')
    } 
}

function main() {
    let ac = 'on'
    if (ac === 'on') {
        console.log('ac is on')
        electricity()
        if (temperature > 25) {
            console.log("it's hot")
        } else {
            console.log("it's cold")
        }
    } else {
        console.log('ac is off')
    }
    service(temperature)
}
