// read all books in a month. Print the completion date of each book.
function main() {
    let number_of_books = 5; //user input
    // let number_of_books = 6; //user input 
    let book = 1;
    let day = 1;
    // let day = 31;
    let month = 1;
    let per_book_day = day / number_of_books;

    for (i = 0;i<number_of_books;i++) {
        console.log("Start reading a book {} Date: {}/01/2023",book, day);
        day +=10;
        if(day>31){
        console.log("Start reading a book {} Date: {}/{}/2023",book, day,month);
        day += per_book_day ;
        if(day>31 && month<13){
            day=day%31;
            month+=1;
        }
        console.log("Complete reading a book {} Date: {}/01/2023",book, day-1);
        console.log("Complete reading a book {} Date: {}/{}/2023",book, day-1,month);
        book += 1;
    }
}    
}  
