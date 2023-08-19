//Check if Today is holiday or not. If Today is not Holiday then go to office.
const today = 'saturday' // user input
switch (today) {
    case 'saturday':
    case 'sunday': {
        console.log('Weekend Holiday')
        break
    }
    case 'special_occasion': {
        console.log('Special occasion Holiday')
        break
    }
    default: {
        console.log('Not Holiday Go to office')
    }
}
