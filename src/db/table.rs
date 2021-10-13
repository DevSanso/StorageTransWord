
use rusqlite::{Connection,Error};


struct Word {
    id : i32,
    chapter : i32,
    page : i32,
    origin_text : String,
    trans_text : String
}


impl Word {
    pub fn Push(conn : &Connection,word : Word) {
        const ext_sql : &str = format!("EXISTS(SELECT * FROM word WHERE origin_text ={});",word.origin_text);
        let count = conn.prepare(ext_sql).unwrap();
    
    }

    pub fn Pop() {

    }
}