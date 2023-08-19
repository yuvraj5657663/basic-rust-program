// step 1: amount=1,00,000 for intrest=4% is 4,000 permonth,so 5days=1,00,666.66.
// total 1,00,666.66 - 25000 = 75,666.66
// step 2: amount=75,666.66 for intrest=2% is 1,513.33 permonth,so 5days=75,918.88
//         total 75,918.88 + 30000 = 1,05,918.88
// step 3: amount=1,05,918.88 for intrest=5% is 5,295.94 permonth,so 5days=1,06 ,800.53
//         total 1,06,800.53 -75000
// step 4: amount=31,800 for intrest=10% is 3,180 permonth,so 15days=33,390.
// (amount+intrest*day/30)

function credit(a) {
    let credit = prompt('Enter amount:')
    let total = Number(a) + Number(credit)
    console.log('Amount:', total)
}
function debit(a) {
    let debit = prompt('Enter amount:')
    let total = Number(a) - Number(debit)
    console.log('Amount:', total)
}
function main() {
    let amount = prompt('Enter initial amount: ')
    let iteration = prompt('Enter how many transitions you want: ')
    for (n = 0; n < iteration; n++) {
        let credit_debit = prompt('Enter 1 for credit and 2 for debit: ')
        let interest_rate = prompt('Enter interest rate(as a decimal): ')
        let days = prompt('Enter number of days: ')
        let a = Number((amount * interest_rate * days) / 30) + Number(amount)
        if (credit_debit === '1') {
            credit(a)
        } else if (credit_debit === '2') {
            debit(a)
        } else {
            console.log('Invalid!')
        }
    }
    console.log('Thank You!!!')
}
main()
