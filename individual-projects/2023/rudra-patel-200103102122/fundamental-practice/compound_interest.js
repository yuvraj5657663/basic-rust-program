function cradit(amount, intrest, days) {
    let cradit = prompt('enter amount:')
     let value = (Number(amount*intrest*days/30) + Number(amount));
     let new_intrest = (Number(value) + Number(cradit));
     console.log('Final:',new_intrest)
    // return five_days + amount;
}
function debit(amount, intrest, days) {
     let debit = prompt('enter amount:')
     let value = (Number(amount*intrest*days/30) + Number(amount));
     let new_intrest = (Number(value) - Number(debit));
     console.log('Final:',new_intrest)
    // return five_days - amount;
}


function main(){
    let amount = prompt('enter your amount: ')
for (let i = 0; i < 4; i++) {
    let user_want_to = prompt('enter cradit = 1 || debit = 2: ');
    let intrest = prompt('enter your intrest: ')
    let days = prompt('enter your days: ')
    
    if (user_want_to == "1") {
        cradit(amount, intrest, days);
    } else if (user_want_to == "2") {
        debit(amount, intrest, days);
    } else {
        
    }
}
}
main();