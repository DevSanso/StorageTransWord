
use rusqlite::{Connection,params};
use std::fmt::{Display,Formatter,Result};



struct Book {
    book_id : i32,
    name : String
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "book_id : {}, name : {}",self.book_id,self.name)
    }
}

impl Book {

    pub fn push(conn : &Connection,name : String) -> rusqlite::Result<()> {
        const INSERT_Q : &str = "INSERT INTO book(name) VALUES(?1);";
        match conn.execute(INSERT_Q, [name]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }

    pub fn pop(conn : &Connection,id : i32) -> rusqlite::Result<()> {
        const DEL_W_Q : &str = "DELETE FROM word WHERE book_id=?1;";
        match conn.execute(DEL_W_Q, [id]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }?;


        const DEL_Q : &str = "DELETE FROM book WHERE book_id=?1;";
        match conn.execute(DEL_Q, [id]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
    pub fn list(conn :&Connection) -> rusqlite::Result<Vec<Book>> {
        const SEL_Q : &str = "SELECT * FROM book;";
        let mut stmt = conn.prepare(SEL_Q)?;
        let iter = stmt.query_map([],|row|  Ok(Book {
            book_id : row.get(0)?,
            name : row.get(1)?
        }))?;
        let mut v  : Vec<Book> = Vec::new();
        iter.for_each(|x| v.push(x.unwrap()));

        Ok(v)
    }
    
}

struct Word {
    book_id : i32,
    chapter : i32,
    page : i32,
    origin_text : String,
    trans_text : String
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "bookID : {}, chapter : {}, page : {}, origin_text : {},trans_text : {}", 
        self.book_id,self.chapter,self.page,self.origin_text,self.trans_text)
    }
}


impl Word {
    fn new(book_id : i32,chapter : i32,page : i32,origin_text : String ,trans_text : String) -> Word {
        Word {
            book_id : book_id,
            chapter : chapter,
            page : page,
            origin_text : origin_text,
            trans_text : trans_text
        }
    }
   fn push_new_data(conn : &Connection,word : Word) -> rusqlite::Result<()> {
        const insert_sql : &str = "INSERT INTO word(book_id,chapter,page,origin_text,trans_text) VALUES(?1,?2, ?3, ?4, ?5);";
        let res = conn.execute(insert_sql, 
            params![word.book_id,word.chapter,word.page,word.origin_text.clone(),word.trans_text.clone()]);

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
    fn change_data(conn : &Connection,word : Word) -> rusqlite::Result<()> {
        const upd_sql : &str = "UPDATE word SET chapter=?1,page=?2,origin_text=?3,trans_text=?4  WHERE book_id=?5 AND chapter=?6 AND origin_text=?7;";
        let res = conn.execute(upd_sql, 
            params![word.chapter,word.page,word.origin_text.clone(),word.trans_text,word.book_id,word.chapter,word.origin_text.clone()]);

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
    fn exist_data(conn : &Connection,word : &Word) -> usize{
        const ext_sql : &str = "SELECT EXISTS(SELECT * FROM word WHERE origin_text =?1 AND book_id=?2);";
        let count : usize = conn.query_row(ext_sql, 
            params![word.origin_text.clone(),word.book_id], 
            |x| x.get(0)
        ).unwrap();
        count
    }

    pub fn push(conn : &Connection,word : Word) -> rusqlite::Result<()> {
        let count = Word::exist_data(&conn, &word);
        if count != 0 {
            return Word::change_data(&conn,word);
        }
        Word::push_new_data(&conn, word)
    }
    pub fn get_word_in_book_and_chapter(conn : &Connection,book_id : i32,chapter : i32) -> rusqlite::Result<Vec<Word>> {
        const Q : &str = "SELECT page,origin_text,trans_text FROM word WHERE book_id = ?1 AND chapter=?2;";
        let mut stmt = conn.prepare(Q)?;
        let iter = stmt.query_map([book_id,chapter], |row| Ok(Word {
            book_id : book_id,
            chapter : chapter,
            page : row.get(0)?,
            origin_text : row.get(1)?,
            trans_text : row.get(2)?
        }))?;
        let mut v : Vec<Word> = Vec::new();
        iter.for_each(|x| v.push(x.unwrap()));
        Ok(v)
    }


    pub fn pop(conn : &Connection,word : Word) -> rusqlite::Result<()> {
        const del_sql : &str = "DELETE FROM word WHERE book_id=?1 AND chapter=?2 AND origin_text=?3;";
        match conn.execute(del_sql, params![word.book_id,word.chapter.clone(),word.origin_text.clone()]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }

    }
}

#[cfg(test)]
mod tests {
    use rusqlite;
    use crate::db::init::init_db;
    fn push_book(conn : &rusqlite::Connection)  -> rusqlite::Result<()>  {
        super::Book::push(&conn, String::from("hello"))?;
        super::Book::push(&conn, String::from("hi"))?;
        super::Book::push(&conn, String::from("bye"))?;

        Ok(())
    }
    #[test]
    fn book_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn)?;

        push_book(&conn)?;
        
        
        let mut v = super::Book::list(&conn)?;
        println!("push");
        for x in v {
            println!("{}", x);
        }
        println!("\n\n\n");
        super::Book::pop(&conn, 1)?;
        v = super::Book::list(&conn)?;
        println!("pop");
        for x in v {
            println!("{}", x);
        }
        Ok(())
    }


    #[test]
    fn table_exist_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        push_book(&conn);
        let data = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        let count = super::Word::exist_data(&conn, &data);
        
        println!("count : {}",count);
        Ok(())
    }
    #[test]
    fn table_push_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        push_book(&conn);
        let data = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        let d = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        super::Word::push_new_data(&conn, data)?;
        let count = super::Word::exist_data(&conn, &d);
        
        println!("count : {}",count);
        Ok(())
    }
    #[test]
    fn table_update_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        push_book(&conn);
        let mut data = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        let mut d = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        super::Word::push_new_data(&conn, data)?;

        data = super::Word::new(1,0,1,String::from("df"),String::from("sdf1223"));
        d = super::Word::new(1,0,1,String::from("df"),String::from("sdf1223"));
        super::Word::change_data(&conn,data)?;
        let res = conn.query_row("SELECT * FROM word;",[], |x| Ok(super::Word {
            book_id : x.get(0).unwrap(),
            chapter : x.get(1).unwrap(),
            page : x.get(2).unwrap(),
            origin_text : x.get(3).unwrap(),
            trans_text : x.get(4).unwrap()
        })).unwrap();

        println!("{}",res);
        
        
        Ok(())
    }
    #[test]
    fn table_pop_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        push_book(&conn);
        let mut data = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        
        super::Word::push(&conn, data)?;
        data = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        let res = conn.query_row("SELECT * FROM word;",[], |x| Ok(super::Word {
            book_id : x.get(0).unwrap(),
            chapter : x.get(1).unwrap(),
            page : x.get(2).unwrap(),
            origin_text : x.get(3).unwrap(),
            trans_text : x.get(4).unwrap()
        })).unwrap();
        println!("{}",res);

        println!("before : {}",super::Word::exist_data(&conn, &data));

        super::Word::pop(&conn, data)?;
        data = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
       

        println!("after : {}",super::Word::exist_data(&conn, &data));
        

        Ok(())
    }
    #[test]
    fn table_list_test() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        init_db(&conn);
        push_book(&conn);

        let mut data = super::Word::new(1,0,0,String::from("df"),String::from("sdf"));
        super::Word::push_new_data(&conn, data)?;
        data= super::Word::new(1,0,0,String::from("df22"),String::from("sdf12"));
        super::Word::push_new_data(&conn, data)?;

        let v = super::Word::get_word_in_book_and_chapter(&conn, 1, 0)?;
        for x in v {
            println!("{}",x);
        }

        Ok(())
    }

}