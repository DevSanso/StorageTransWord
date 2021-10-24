use std::io::{self,BufRead};

use crate::var::Var;
use crate::db::table::Book;
use super::super::component::BookList;
use super::View;

pub struct BookListView {}

impl BookListView {
    pub fn new() -> BookListView {
        BookListView{

        }
    }
    fn get_list(&self) -> Vec<Book> {
        let conn = Var::get_db_conn_as_mut_ref();
        Book::list(&conn).unwrap()
    }
}


impl View for BookListView {
    fn display(&self) -> io::Result<()> {
        println!("{}",BookList::new(self.get_list()));
        println!("press enter");
        Ok(())
    }
    fn input(&mut self) -> io::Result<()>{
        let mut b = String::new();
        io::stdin().read_line(&mut b);
        Ok(())
    }
    fn update(&mut self) -> io::Result<()>{
        Ok(())
    }
    fn is_more_run(&self) -> bool {
        false
    }
    fn next<'parent>(&self) -> Option<Box<dyn View + 'parent>> {
        None
    }
}