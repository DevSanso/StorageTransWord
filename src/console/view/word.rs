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
    num_temp : Option<i32>,
    err_message : Option<String>

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
            num_temp : None,
            err_message : None
        };
        w.update_word_list();
        w
    }
    fn make_empty_word(&self) -> Word {
        Word::new(self.book_id,self.chapter,0,String::new(),String::new())
    }
    fn input_num_temp(&mut self) {
        let mut buf = String::new();
        io::stdin().lock().read_line(&mut buf);
        buf = buf.replace("\n", "");
        let casting = buf.parse::<i32>();
        if casting.is_err() {
            self.input_result = Err(String::from("숫자만 입력하세요"));
            self.num_temp = None;
            return;
        }
        self.num_temp = Some(casting.unwrap());
    }
    fn stdin_read_page_and_write_word_struct(&mut self,w : &mut Word) -> Result<(),&'static str>{
        let temp = self.num_temp;
        println!("페이지(숫자)를 입력하세요 : ");
        self.input_num_temp();

        if self.num_temp.is_none() {
            self.num_temp = temp;
            return Err("숫자만 입력하세요");
        }
        w.page = self.num_temp.unwrap();
        self.num_temp = temp;

        Ok(())
    }
    fn stdin_read_en_word_and_write_word_struct(&self,w : &mut Word) -> Result<(),&'static str> {
        println!("영어 단어를 입력하세요 : ");
        io::stdin().lock().read_line(&mut w.origin_text);
        Ok(())
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
    fn input_change_chapter(&mut self) {
        println!("챕터(숫자)를 입력하세요 : ");
        self.input_num_temp();
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
    fn update_input_result_None(self : &mut Self) {
        self.input_result = Ok(None);
    }

    fn if_input_err_and_update(&mut self) -> bool {
        let mut res =  self.input_result.clone();

        if res.is_err() {
            self.num_temp = Some(-1);
            self.sub_update_next_action();
            self.err_message = Some(res.err().unwrap());
            self.input_result = Ok(None);
            return true;
        }

        false
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
        if self.action == Action::Insert || self.action == Action::Delete{
            let mut w = self.make_empty_word();
            if self.action == Action::Insert  {
                let r = self.stdin_read_page_and_write_word_struct(&mut w);
                if r.is_err() {
                    self.input_result = Err(String::from(r.err().unwrap()));
                    return Ok(());
                }
                
            }

            self.input_result = match self.stdin_read_en_word_and_write_word_struct(&mut w) {
                Ok(ok) => Ok(Some(w)),
                Err(err) => Err(String::from(err))
            };
            
            return Ok(());
        }
        else if self.action == Action::ChangeChapter {
            self.input_change_chapter();
        }
        
        else {self.input_num_temp();}

        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        if self.if_input_err_and_update() {
            return Ok(());
        }
        self.err_message = None;
        if self.action == Action::ChangeChapter {
            self.chapter = self.num_temp.unwrap();
            self.num_temp = Some(-1);
        }
        
        if self.action == Action::Insert {
            let w = self.input_result.clone().unwrap().unwrap();
            let insert_res = self.insert_word(w);
            if insert_res.is_err() {
                self.input_result = Err(String::from(insert_res.err().unwrap()))
            }
            self.num_temp = Some(-1);
        }
        else if self.action == Action::Delete {
            let w = self.input_result.clone().unwrap().unwrap();
            
            let del_res = self.delete_word(w);
            if del_res.is_err() {
                self.input_result = Err(String::from(del_res.err().unwrap()))
            }
            self.num_temp = Some(-1);
        }
        if self.if_input_err_and_update() {
            return Ok(());
        }

        self.sub_update_next_action();


        if self.action == Action::Exit {
            self.is_close = true;
            return Ok(());
        }


        self.update_word_list();
        Ok(())
    }
    fn is_more_run(&self) -> bool {
        !self.is_close
    }
    fn next<'parent>(&self) -> Option<Box<dyn View + 'parent>> {
        if self.err_message.is_some() {
            let msg = self.err_message.clone().unwrap();
            
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