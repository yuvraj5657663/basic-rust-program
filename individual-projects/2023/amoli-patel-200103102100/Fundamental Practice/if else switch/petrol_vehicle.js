//Check the petrol or diesel of your vehicle
// for type of vehicle
const type = 'Diesel'
switch (type) {
    case 'Petrol': {
        console.log('Petrol')
        break
    }
    case 'Diesel': {
        console.log('Diesel')
        break
    }
    default: {
        console.log('Invalid')
    }
}

// for fuel indication
const indication = 'Empty'
switch (indication) {
    case 'Full': {
        console.log('Full')
        break
    }
    case 'Empty': {
        console.log('Empty')
        break
    }
    default: {
        console.log('Average')
    }
}
