// update all applications of a device - use loop and function
function logic() {
    let update = false
    for (n = 0; n < 5; n++) {
        if (update === true) {
            console.log('update')
        } else {
            console.log('no update available')
        }
    }
}
function main() {
    logic()
}
main()
