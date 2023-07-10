// Check all pens in the box are working or not. If not working then remove it from box and print total working pens.
function main() {
    let working = 0,
        not_working = 2 // assume pen not working
    for (pens = 1; pens <= 5; pens++) {
        if (pens === not_working) {
            console.log('removed')
            continue
        }
        console.log('pen', pens, 'is working')
        working++
    }
    console.log('total working pens are', working)
}
main()
