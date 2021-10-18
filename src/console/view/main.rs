use std::io::{self,BufRead};
use rusqlite::Connection;


use crate::var::Var;
use crate::db::table::Book;
use crate::console::view::err::{ErrorView,ErrorStringView};
use crate::console::view::make_book::MakeBookView;
use super::super::component::{Title,BookList};




struct MainMenu {
    is_close : bool,
    title : Title,
    list : BookList,

    input_data : Result<i32,std::num::ParseIntError>,
}

impl MainMenu{
    pub fn new() -> MainMenu {
        let conn = Var::get_db_conn_as_mut_ref();
        MainMenu{
            is_close : false,
            title : Title::new(),
            list : BookList::new(Book::list(conn).unwrap()),

            input_data : Result::Ok(0)
        }
    }

    fn make_next_view<'b>(&self,n : i32) -> Option<Box<dyn super::View + 'b>> {
        match n {
            1 => None,
            9 => None,
            _ => Some(Box::new(ErrorStringView::new("only input 1,2,9")))
        }

        
    } 
}


impl super::View for MainMenu{
    fn display(&self) -> io::Result<()> {
        print!("{}\n",self.title);
        print!("{}\n",self.list);
        print!("입력 => 책 생성 (1), 책 불려오기(2), 종료(9) >>");
        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        let val = self.input_data.clone().unwrap_or(-999);
        if val == 9 {
            self.is_close = true;
        }
        let conn = Var::get_db_conn_as_mut_ref();
        self.list = BookList::new(Book::list(conn).unwrap());
        Ok(())
    }

    fn input(&mut self) -> io::Result<()> {
        let mut buf = String::new();
        io::stdin().lock().read_line(&mut buf).unwrap();
        self.input_data=buf.parse::<i32>();

        Ok(())
    }
    fn is_more_run(&self) -> bool {
        !self.is_close
    }

    fn next<'parent>(&self) -> Option<Box<dyn super::View + 'parent>> {
        let val = self.input_data.clone();
        match val {
            Ok(ok) => self.make_next_view(ok),
            Err(err) => Some(Box::new(ErrorView::new(err)))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read;
    use rusqlite::Connection;
    use super::MainMenu;
    use super::super::View;
    use crate::db::init::init_db;

 
    #[test]
    fn main_menu_test() -> io::Result<()> {
        let c = Connection::open_in_memory().unwrap();
        init_db(&c);
        let mut m  :Box<dyn View>  = Box::new(MainMenu::new());
        m.exec()?;
        m = m.next().unwrap();
        m.exec()?;

        Ok(())
    }
}