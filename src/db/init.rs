use rusqlite::{Connection,Error};


const INIT_TABLE : &str = "CREATE TABLE word(
    book VARCHAR(256) PRIMARY KEY,
    chapter        INTEGER,
    page             INTEGER,
    origin_text   VARCHAR(256),
    trans_text    VARCHAR(256),
    UNIQUE(origin_text)
);";


pub fn  init_db(conn : &Connection) -> Result<usize,Error> {
    conn.execute(INIT_TABLE ,[])
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