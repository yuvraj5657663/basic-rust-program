fn main() {
    let number_of_books = 6; //also take user input here..
    let mut book = 1;
    let mut day = 31;
    let mut month = 1;
    let mut per_book_day = day / number_of_books;

    for _i in 0..number_of_books {
        println!("Start reading a book {} Date: {}/{}/2023",book, day,month);
        day += per_book_day ;
        if(day>31){
            day=day%31;
            month+=1;
        }
        if (month >= 12){
            month = month%12;
        }
        println!("Complete reading a book {} Date: {}/{}/2023",book, day-1,month);
        book += 1;
    }
}  



    // fn main() {
    //     let number_of_books = 5; // also take user input here..
    //     let mut book = 1;
    //     let mut day = 1;
    
    //     for _i in 0..number_of_books {
    //         println!("Start reading a book {} Date: {}/01/2023",book, day);
    //         day +=10;
            
    //         if(day>31){
    //             day=day%31;
    //         }
    //         println!("Complete reading a book {} Date: {}/01/2023",book, day-1);
    //         book += 1;
    //     }
    // }    