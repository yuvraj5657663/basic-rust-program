fn main() {
    let month_days = 30;
    let date = 1;
    let month = 1;
    let year = 2023;
    let books = ["Book-1", "Book-2", "Book-3"];
    
    let days_per_book = month_days / books.length();

    let month_date_counter = 1;

    for i in books {
        print!(" read Book {} on date ", i , );
        month_days_counter += days_per_book;
    }
}