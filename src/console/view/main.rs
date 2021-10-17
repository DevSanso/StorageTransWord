use std::io::{self,BufRead};
use rusqlite::Connection;



use crate::db::table::Book;
use crate::console::view::err::ErrorView;
use super::super::component::{Title,BookList};




struct MainMenu {
    conn : &'static Connection,
    is_close : bool,
    title : Title,
    list : BookList,

    input_data : Result<i32,std::num::ParseIntError>,
}

impl MainMenu {
    pub fn new(conn : &'static Connection) -> MainMenu {
        MainMenu{
            conn : conn,
            is_close : false,
            title : Title::new(),
            list : BookList::new(Book::list(&conn).unwrap()),

            input_data : Result::Ok(0)
        }
    }

    fn make_next_view(&self,n : i32) -> Option<Box<dyn super::View>> {
        if n == 9 {return None;}

        None
    } 
}


impl super::View for MainMenu {
    fn display(&self) -> io::Result<()> {
        println!("{}",self.title);
        println!("{}",self.list);
        print!("입력 => 책 생성 (1), 책 불려오기(2), 종료(9)");
        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        let val = self.input_data.clone().unwrap_or(-999);
        if val == 9 {
            self.is_close = true;
        }
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
    fn next(&self) -> Option<Box<dyn super::View>> {
        let val = self.input_data.clone();
        match val {
            Ok(ok) => self.make_next_view(ok),
            Err(err) => Some(Box::new(ErrorView::new(err)))
        }
    }
}


