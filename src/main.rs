struct BOOK {
  pages: u16,
  rating: f32,
}

fn display_page_count(book: &BOOK) {
  println!("Pages: {}", book.pages);
}

fn display_rating(book: &BOOK) {
  println!("Rating: {}", book.rating);
}

fn main() {
  let book = BOOK {
    pages: 100,
    rating: 4.5,
  };

  display_page_count(&book);
  display_rating(&book);
}
