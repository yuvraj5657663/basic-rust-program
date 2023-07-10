// Write a code to check number of switches and its working status in switch board .Print number of working and not working switches
function main() {
    let working = 0,
        not_working = 0
    for (switches = 0; switches < 5; switches++) {
        if (switches === 2 || switches === 4) {
            console.log('not working')
            not_working++
            continue
        }
        console.log('working')
        working++
    }
    console.log('total switches are', switches)
    console.log('working switches are', working)
    console.log('not working switches are', not_working)
}
main()
