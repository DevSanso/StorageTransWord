use std::io::{self,BufRead};


use super::View;
use super::err::ErrorStringView;
use crate::console::component::WordList;
use crate::var::Var;
use crate::db::table::Word;

#[derive(PartialEq)]
enum Action {
    Insert,
    Delete,
    ChangeChapter,
    Exit,
    Nothing
}



pub struct WordView {
    book_name : String,
    book_id : i32,
    chapter : i32,
    list : Option<WordList>,
    is_close : bool,
    action : Action,

    input_result : Result<Option<Word>,String>,
    num_temp : Option<i32>

}

impl WordView {
    pub fn new(book_name : String,book_id : i32,chapter : i32) -> WordView {
        let mut w = WordView {
            book_name : book_name,
            book_id : book_id,
            chapter : chapter,
            list : None,
            is_close : false,
            action : Action::Nothing,
            input_result : Ok(None),
            num_temp : None
        };
        w.update_word_list();
        w
    }

    fn stdin_read_and_init_word_struct_data(&self) -> Result<Word,&'static str>  {
        let mut w = Word::new(self.book_id,self.chapter,0,String::new(),String::new());
        let mut temp_buf = String::new();
        print!("페이지를 입력하세요(숫자만) : ");
        io::stdin().lock().read_line(&mut temp_buf);
        temp_buf = temp_buf.replace("\n", "");
        let casting = temp_buf.parse::<i32>();
        if casting.is_err() {
            return Err("숫자만 입력하세요");
        }
        w.page = casting.unwrap();

        print!("영어 단어를 입력하세요 : ");
        io::stdin().lock().read_line(&mut w.origin_text);
        Ok(w)
    }
    fn input_num_temp(&mut self) {
        let mut buf = String::new();
        io::stdin().lock().read_line(&mut buf);
        buf = buf.replace("\n", "");
        let casting = buf.parse::<i32>();
        if casting.is_err() {
            self.input_result = Err(String::from("숫자만 입력하세요"));
            return;
        }
        self.num_temp = Some(casting.unwrap());
    }
    fn insert_word(&self, w : Word) -> Result<(),&'static str>{
        let mut temp = w;
        let trans_d = Var::get_driver_as_ref();
        let res = trans_d.trans(temp.origin_text.clone());

        temp.trans_text = res.data;
        let conn = Var::get_db_conn_as_mut_ref();
        if  Word::push(&conn,temp).is_err() {
            return Err("database push error");
        }
        Ok(())
    }
    fn delete_word(&self, w : Word)  -> Result<(),&'static str> {
        let conn = Var::get_db_conn_as_mut_ref();
        if Word::pop(conn,w).is_err() {
            return Err("database pop error");
        }

        Ok(())
    }
    fn change_chapter(&mut self) -> Result<(),&'static str> {
        print!("챕터(숫자)를 입력하세요 : ");
        self.input_num_temp();

        Ok(())
    }

    fn update_word_list(&mut self) {
        let conn = Var::get_db_conn_as_mut_ref();
        let data = Word::get_word_in_book_and_chapter(conn,self.book_id,self.chapter);
        if data.is_err() {
            let errMsg : String = format!("{}",data.err().unwrap()).to_string();
            self.input_result = Err(errMsg.clone());
            return;
        }

        self.list = Some(WordList::new(self.book_name.clone(),self.chapter,data.unwrap()));
    }
    fn sub_update_next_action(&mut self) {
        if self.num_temp.is_none() {
            return;
        }
        let num = self.num_temp.unwrap();
        if num == 1 {self.action = Action::Insert;}
        else if num == 2 {self.action = Action::Delete;}
        else if num == 3 {self.action = Action::ChangeChapter;}
        else if num == 4 {self.action = Action::Exit;}
        else {self.action = Action::Nothing;}
    }
       

}


impl View for WordView {
    fn display(&self) -> io::Result<()> {
        println!("{}",self.list.as_ref().unwrap());
        println!("");
        if self.action == Action::Nothing {
            print!("입력 : 1(단어 추가), 2(단어 삭제), 3(챕터 변경),4(돌아가기) :\n");
        }

        Ok(())
    }
    fn input(&mut self) -> io::Result<()> {
        if self.action == Action::Insert && self.action == Action::Delete{
            self.input_result = match self.stdin_read_and_init_word_struct_data() {
                Ok(ok) => Ok(Some(ok)),
                Err(err) => Err(String::from(err))
            };
            
            return Ok(());
        }
        else {self.input_num_temp();}

        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        self.sub_update_next_action();

        if self.action == Action::Exit {
            self.is_close = true;
            return Ok(());
        }
        else if self.action == Action::ChangeChapter {
            self.chapter = self.num_temp.unwrap();
            self.action = Action::Nothing;
            return Ok(());
        }

    

        let res = self.input_result.clone();
        if res.is_err() {
            return Ok(());
        }
        
        if self.action == Action::Insert {
            let mut w = res.unwrap().unwrap();
            self.insert_word(w);
        }
        else if self.action == Action::Delete {
            let w = res.unwrap().unwrap();
            self.delete_word(w);
        }

        self.action = Action::Nothing;
       
        self.update_word_list();
        Ok(())
    }
    fn is_more_run(&self) -> bool {
        !self.is_close
    }
    fn next<'parent>(&self) -> Option<Box<dyn View + 'parent>> {
        if self.input_result.is_err() {
            let msg = self.input_result.clone().err().unwrap();
            return Some(Box::new(ErrorStringView::new(
                msg
            )));
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read;
    use rusqlite::Connection;
    use super::super::View;
    use super::WordView;
    use crate::db::init::init_db;
    use crate::db::table::Book;
    use crate::var::Var;
    #[test]
    fn word_view_test() {
        Var::test_init();
        let mut v :Box<dyn View> = Box::new(WordView::new(String::from("hello"),1,1));
        let c = Var::get_db_conn_as_mut_ref();
        Book::push(&c,String::from("hello"));
        while v.is_more_run() {

            println!("running");
            v.exec();
            let m = v.next();
            if m.is_some() {
                m.unwrap().exec();
            }
        }
    }
}