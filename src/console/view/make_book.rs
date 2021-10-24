use super::View;
use rusqlite::{Connection,self};
use std::io::{self,BufRead};
use crate::db::table::Book;
use crate::console::component::BookList;
use crate::console::view::err::{ErrorView,ErrorStringView};
use crate::var::Var;
pub struct MakeBookView{
    list : Option<BookList>,
    is_done : bool,
    input_data :String,

    insert_res : rusqlite::Result<()>
}



impl MakeBookView {
    pub fn new() -> MakeBookView {
        let mut b = MakeBookView{
            list : None,
            is_done : false,
            input_data : String::new(),
            insert_res : Ok(())
        };
        b.update_list();
        b
    }
    fn update_list(&mut self) {
        let conn = Var::get_db_conn_as_mut_ref();
        let g = Book::list(conn).unwrap();
        self.list = Option::Some(BookList::new(g));
    }


    fn insert_book(&self) -> rusqlite::Result<()>{
        let s = self.input_data.clone();
        let conn = Var::get_db_conn_as_mut_ref();
        Book::push(conn,s)
    }
}

impl View for MakeBookView {
    fn display(&self) -> io::Result<()> {
        let l = self.list.as_ref();
        println!("{}",l.unwrap());
        println!("\n\n책이름 입력 : ");
        Ok(())
    }
    fn input(&mut self) -> io::Result<()> {
        let mut buf = String::new();
        io::stdin().lock().read_line(&mut buf).unwrap();
       
        self.input_data=buf.replace("\n", "");
        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        self.update_list();
        self.insert_res = self.insert_book();
        
        self.is_done = true;
        Ok(())
    }
    fn is_more_run(&self) -> bool {
        !self.is_done
    }
    fn next<'b>(&self) -> Option<Box<dyn View + 'b>> {
        if self.insert_res.is_ok() {
            None
        }
        else {
            Some(Box::new(ErrorStringView::new_str("error, that input book name in db\n")))
        }
    }
}