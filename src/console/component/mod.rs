pub trait Component {}



mod print_list;
mod title;

pub use title::Title;
pub use print_list::BookList;
pub use print_list::WordList;