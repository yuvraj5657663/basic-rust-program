// ATM Program - Deposit and withdraw function
// notes count with condition
function checkbalance() {
    let balance = [20, 50, 10]
    current_balance(balance)
    return balance
}
function current_balance(balance) {
    console.log(
        'No of 2000rs notes=',
        balance[0],
        ', 500rs notes=',
        balance[1],
        'and 100rs notes=',
        balance[2]
    )
}
function deposite(amount, balance) {
    const n_t2000 = balance[0] + amount / 2000
    return [n_t2000, balance[1], balance[2]]
}
function withdraw(amount, balance) {
    const n_t500 = balance[1] - amount / 500
    if (n_t500 < 0) {
        const n_t100 = balance[2] - amount / 100
        if (n_t100 < 0) {
            return [balance[0], 'Sorry!!', 'Sorry!!']
        } else {
            return [balance[0], balance[1], n_t100]
        }
    } else {
        return [balance[0], n_t500, balance[2]]
    }
}

function main() {
    let balance = checkbalance()
    balance = deposite(10000, balance)
    balance = withdraw(5000, balance)
    current_balance(balance)
}
main()
