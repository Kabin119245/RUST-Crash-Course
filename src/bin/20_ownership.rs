struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book ) {
    println!("pages = {:?}", book.pages);
}


fn display_rating(book: &Book ) {
    println!("rating = {:?}", book.rating);
}


fn main() {
    let book = Book {
        pages: 125,
        rating: 9,
    };

   // display_page_count(book); //once this function is executed book will not be availabe below this line
   //so we have to borrow data using & symbol
   display_page_count(&book);
    display_rating(&book);
}