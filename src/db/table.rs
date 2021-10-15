#[macro_export]
use rusqlite::{Connection,params};
use std::fmt::{Display,Formatter,Result};

struct Word {
    book : String,
    chapter : i32,
    page : i32,
    origin_text : String,
    trans_text : String
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "book : {}, chapter : {}, page : {}, origin_text : {},trans_text : {}", 
        self.book,self.chapter,self.page,self.origin_text,self.trans_text)
    }
}


impl Word {
    fn new(book : String,chapter : i32,page : i32,origin_text : String ,trans_text : String) -> Word {
        Word {
            book : book,
            chapter : chapter,
            page : page,
            origin_text : origin_text,
            trans_text : trans_text
        }
    }
   fn push_new_data(conn : &Connection,word : Word) -> rusqlite::Result<()> {
        const insert_sql : &str = "INSERT INTO word(book,chapter,page,origin_text,trans_text) VALUES(?1,?2, ?3, ?4, ?5);";
        let res = conn.execute(insert_sql, 
            params![word.book.clone(),word.chapter,word.page,word.origin_text.clone(),word.trans_text.clone()]);

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
    fn change_data(conn : &Connection,word : Word) -> rusqlite::Result<()> {
        const upd_sql : &str = "UPDATE word SET chapter=?1,page=?2,origin_text=?3,trans_text=?4  WHERE origin_text=?5 AND book=?6;";
        let res = conn.execute(upd_sql, 
            params![word.chapter,word.page,word.origin_text.clone(),word.trans_text,word.origin_text.clone(),word.book.clone()]);

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
    fn exist_data(conn : &Connection,word : &Word) -> usize{
        const ext_sql : &str = "SELECT EXISTS(SELECT * FROM word WHERE origin_text =?);";
        let count : usize = conn.query_row(ext_sql, 
            [word.origin_text.clone()], 
            |x| x.get(0)
        ).unwrap();
        count
    }

    pub fn Push(conn : &Connection,word : Word) -> rusqlite::Result<()> {
        let count = Word::exist_data(&conn, &word);
        if count != 0 {
            return Word::change_data(&conn,word);
        }
        Word::push_new_data(&conn, word)
    }



    pub fn Pop() {


    }
}

#[cfg(test)]
mod tests {
    use rusqlite;
    use crate::db::init::init_db;
    #[test]
    fn table_exist_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        let data = super::Word::new(String::from("book"),0,0,String::from("df"),String::from("sdf"));
        let count = super::Word::exist_data(&conn, &data);
        
        println!("count : {}",count);
        Ok(())
    }
    #[test]
    fn table_push_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        let data = super::Word::new(String::from("book"),0,0,String::from("df"),String::from("sdf"));
        let d = super::Word::new(String::from("book"),0,0,String::from("df"),String::from("sdf"));
        super::Word::push_new_data(&conn, data)?;
        let count = super::Word::exist_data(&conn, &d);
        
        println!("count : {}",count);
        Ok(())
    }
    #[test]
    fn table_update_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        let mut data = super::Word::new(String::from("book"),0,0,String::from("df"),String::from("sdf"));
        let mut d = super::Word::new(String::from("book"),0,0,String::from("df"),String::from("sdf"));
        super::Word::push_new_data(&conn, data)?;

        data = super::Word::new(String::from("book"),0,1,String::from("df"),String::from("sdf123"));
        d = super::Word::new(String::from("book"),0,1,String::from("df"),String::from("sdf1223"));
        super::Word::change_data(&conn,data)?;
        let res = conn.query_row("SELECT * FROM word;",[], |x| Ok(super::Word {
            book : x.get(0).unwrap(),
            chapter : x.get(1).unwrap(),
            page : x.get(2).unwrap(),
            origin_text : x.get(3).unwrap(),
            trans_text : x.get(4).unwrap()
        })).unwrap();

        println!("{}",res);
        
        
        Ok(())
    }

  
}