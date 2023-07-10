// function nots(){
//     let two_thousand = 50;
//     let five_hundred = 50;
//     let hundred = 50;
//     let deposit = 10000;
//     let withdraw = 5000;
// }

// function balence(){
//     let a = two_thousand*2000;
//     let b = five_hundred*500;
//     let c = hundred*100;
// }

// function cradit(balence, deposit){
//    return a + deposit
// }
// function debit()

// function check_balence() {
//     let balence = 25000;
//     current_balence(balence);
//     return balence;
//   }
//   function current_balence(balence) {
//     console.log("new_balence", balence);
//   }

//   function cradit(balence, amount) {
//     return balence + amount;
//   }

//   function debit(balence, amount) {
//     return balence - amount;
//   }

//   function pin() {
//     const pin = 1425;
//   }
//   function change_pin() {
//     pin();
//     const new_pin = 5244;
//     console.log(new_pin);
//   }

//   function main() {
//     let balence = check_balence();
//     balence = cradit(balence, 5000);
//     balence = debit(balence, 10000);
//     balence = cradit(balence, 3000);
//     current_balence(balence);
//     change_pin();
//   }

//   main();

// function check_notes() {
//     let two_thousand = 50;
//     let five_hundred = 50;
//     let one_hundred = 50;

//     return current_notes(two_thousand,five_hundred,one_hundred);
// }

// function current_notes(two_thousand,five_hundred,one_hundred) {
//     console.log("new_notes", two_thousand, five_hundred, one_hundred);
//     return [two_thousand,five_hundred,one_hundred]
// }

// function cradit(two_thousand, amount) {
//     return two_thousand + amount;
// }

// function debit(two_thousand, amount, five_hundred, amount) {
//     return two_thousand - amount;
//     return five_hundred - amount;
// }

// function main(){
//     let notes = check_notes();
//     let two_thousand = notes[0]
//     let five_hundred = notes[1]
//     notes = cradit(two_thousand, 5);
//     notes = debit(two_thousand, 2, five_hundred, 2);
// }

// main();











// function notes() {
//     let two_thousand = 50;
//     let five_hundred = 50;
//     let one_hundred = 50;
//     current_ammount(notes);
//     return notes;
// }

// function cradit(notes, amount) {
//     return (w = two_thousand + amount * 2000)
//     return (x = two_thousand + amount)
// }

// function debit(notes, amount) {
//     return (y = two_thousand - amount * 2000)
//     return (y_note = two_thousand - amount)
//     return (z = five_hundred - amount * 500)
//     return (z_note = five_hundred - amount)
// }


// function main(){
//     let current_ammount = notes;
//     current_ammount = cradit(current_ammount, 5)
//     current_ammount = debit(current_ammount, 2)
// }

// main();


   


// let arr = [50, 50, 50]

// function cradit(a, b, c){
//     return [arr[0] + a, arr[1] + b, arr[2] + c]
// }

// function debit(x, y, z){
//     return [arr[0] - x, arr[1] - y, arr[2] - z]
// }

// function main(){
//     arr = cradit(5, 0, 0);
//     arr = debit(2, 2 , 0);
// }
//  main();









// let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// let arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// let totalCards = 20;
// let personAPoints = 0;
// let personBPoints = 0;


// function personA (arr1[a], arr2[b]) {
//   if (arr1 == arr2) {
//      personAPoints++; 
//      totalCards = totalCards - 2;
//   } 
// }

// function personB (arr1, arr2) {
//   if (arr1 == arr2) { 
//      personBPoints++;
//      totalCards = totalCards - 2;
//   }
// }

// function winner (personAPoints, personBPoints) {
//     if (personAPoints < personBPoints){
//         console.log ("personB win")
//      }
//      else if (personAPoints > personBPoints){
//         console.log ("personA win")
//      }
//      else {
//         console.log ("drow the match")
//       }
//    }







let deck = [1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,0,0]
let personACards = {
0: [3,4],
1: [4,5],
2: [3,2],
3: [6,7],
4: [7,8]
}
let personBCards = {
   0: [2,3],
   1: [5,6],
   2: [3,2],
   3: [8,9],
   4: [7,8]
   }
let personAPoints = 0;
let personBPoints = 0;
let n = 0;

function turn(pA, pB){

   if (n % 2 === 1){
      console.log ("personA turn");
      const roundCards = personACards[n]
      n++;
   }
   else if (n % 2 === 0){ 
      console.log ("personB turn");
      const roundCards = personACards[n]
      n++;
   }
   else{
      
   }

}

function personA (a,b) {
  if (a == b){
     personAPoints++; // personAPoints = personAPoints + 1
     totalCards = totalCards - 2;
   } 
}

function personB (x,y) {
   if (x == y) {
      personBPoints++;
      totalCards = totalCards - 2;
   }
}
function main(){
   turn();
  let totalCards = 20;
  let totalRounds = 5;
  
 let pA = personA(3,4); // a0
 let pB = personB(3,3); // b1
    
  personA(7,8); // a0
  personB(4,5); // b1
  
  personA(4,4); // a1
  personB(5,5); // b2
  
  personA(8,8); // a2
  personB(6,9); // b2
  
  personA(0,9); //a2
  personB(6,6); //b3
  
  if (personAPoints < personBPoints){
           console.log ("personB win")
        }
        else if (personAPoints > personBPoints){
           console.log ("personA win")
        }
        else {
           console.log ("drow the match")
         }
}