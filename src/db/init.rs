use rusqlite::{Connection,Error};



fn chk_exist_table(conn : &Connection,name : &str) -> bool {
    const Q : &str = "SELECT COUNT(*) FROM sqlite_master WHERE type=\"table\" AND name=?1;";
    let count  : i32 = conn.query_row(Q, [name], |x| x.get(0)).unwrap();
    if count != 0 {
        true
    }else {
        false
    }
}

pub fn  init_db(conn : &Connection) -> Result<usize,Error> {
    const INIT_BOOK_TABLE : &str = "CREATE TABLE book(
        book_id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(256)  
    );";
    
    const INIT_WORD_TABLE : &str = "CREATE TABLE word(
        book_id       INTERER,
        chapter        INTEGER,
        page             INTEGER,
        origin_text   VARCHAR(256),
        trans_text    VARCHAR(256),
        UNIQUE(origin_text),
        FOREIGN KEY(book_id) REFERENCES book(book_id)
    );";
    conn.execute("PRAGMA foreign_keys=1;", [])?;

    if !chk_exist_table(&conn, "book") {
        conn.execute(INIT_BOOK_TABLE, [])?;
    }

    if !chk_exist_table(&conn, "word") {
        conn.execute(INIT_WORD_TABLE ,[])?;
    }

  
    Ok(0)
}



#[cfg(test)]
mod tests {
    use rusqlite;
    use rusqlite::{Connection};
    #[test]
    fn test_init() -> rusqlite::Result<()> {
        let conn = Connection::open_in_memory()?;
        super::init_db(&conn)?;
        Ok(())
        
    }
}