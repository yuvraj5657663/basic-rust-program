function checkbalance() {
  let balance = 25000;
  current_balance(balance);
  return balance;
}
function current_balance(balance) {
  console.log("new_balance", balance);
}
function cradit(balance, amount) {
  return balance + amount;
}
function debit(balance, amount) {
  return balance - amount;
}
function pin() {
  const pin = 1425;
}
function change_pin() {
  pin();
  const new_pin = 5244;
  console.log(new_pin);
}
function main() {
  let balance = checkbalance();
  balance = cradit(balance, 5000);
  balance = debit(balance, 10000);
  balance = cradit(balance, 3000);
  current_balance(balance);
  change_pin();
}
main();