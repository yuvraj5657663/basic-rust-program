// books read in a month

function main() {
  let books = [
    { name: 'JavaScript', pages: 140 },
    { name: 'CSS', pages: 90 },
    { name: 'HTML', pages: 70 }
  ];

  let total = 0;
  for (let i = 0; i < books.length; i++) {
    total += books[i].pages;
  }

  let day = 30;
  console.log(total / day);
  date();
}
function date(){
      let date = 1;
     for (let i = 1; i <= 30; i++){
      if ( i <= 10){
          console.log("You have 30 days for first book")
          i += 1;
      }else if ( i <= 20){
          console.log("You have 20 days for secoend book")
          i += 1;
      }else if (i <= 30){
          console.log("You have 10 days for third book")
          i += 1;
      }else{
          console.log("You can read 4 books in a month")
      }
  }
}
main();














// function main() {
//   let number_of_books = 6;
//   let read_book = 1;
//   let date = 30;
//   let month = 4;
//   let year = 2023;
//   let days_per_book = date / number_of_books;

//   for (i = number_of_books; i>=1; i--) {
//       console.log(`Start reading a book ${read_book} Date: ${date}/${month}/${year}`);
//       date += days_per_book ;
//       if(date > 30){
//           date = date % 30;
//           month ++;
//       }
//       else if(month > 12){
//           month = month % 12;
//           year ++;
//       }
//       console.log(`Complete reading a book ${read_book} Date: ${date-1}/${month}/${year}`);
//       read_book ++;
//   }
// }  
// main();