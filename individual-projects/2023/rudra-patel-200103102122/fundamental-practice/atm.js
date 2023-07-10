// atm machine

function check_balence() {
    let balence = 25000;
    current_balence(balence);
    return balence;
  }
  function current_balence(balence) {
    console.log("new_balence", balence);
  }
  
  function cradit(balence, amount) {
    return balence + amount;
  }
  
  function debit(balence, amount) {
    return balence - amount;
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
    let balence = check_balence();
    balence = cradit(balence, 5000);
    balence = debit(balence, 10000);
    balence = cradit(balence, 3000);
    current_balence(balence);
    change_pin();
  }
  
  main();
  
  