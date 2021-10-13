use rusqlite::{Connection,Error};


const INIT_TABLE : &str = "CREATE TABLE word (
    id                  INTEGER AUTO INCREMENT,
    chapter        INTEGER
    page             INTEGER,
    origin_text   VARCHAR(256),
    trans_text    VARCHAR(256)
    PRIMARY KEY(id),
    UNQIUE(origin_text)
)";


pub fn  init_db(conn : &Connection) -> Result<usize,Error> {
    conn.execute(INIT_TABLE ,[])
}