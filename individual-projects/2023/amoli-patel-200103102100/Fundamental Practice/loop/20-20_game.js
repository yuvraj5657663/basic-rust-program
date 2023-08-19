//Create a loop for 20 20 game. Stop the game if its raining
function main() {
    let n = 0,
        rain // n is a over of match
    while (n < 20) {
        if (rain == true) {
            let total_over = 20
            while (n <= total_over) {
                if (rain == true) {
                    // random over this argument is true
                    console.log('Stop rain is start..')
                    break
                } else {
                    console.log('Play Game...')
                }
                n += 1
            }

            rain = false
            while (n <= total_over) {
                console.log('Continue after rain...')
            }
        }
    }
}
