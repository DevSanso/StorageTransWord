use std::include_str;
use std::path::Path;
use std::fs::read_to_string;
use rusqlite::Connection;

mod driver;
mod db;
mod console;

use db::init::init_db;
const DBNAME : &str = "words.db";


fn read_config() -> String {
    let p = Path::new("./").join("./driver_config.json");
    read_to_string(p).unwrap()
}

fn main()  { 
    let db_path = Path::new("./").join(DBNAME);
    let driver_config = read_config();
    let conn = Connection::open(db_path);

    let trans_driver = driver::facory(driver_config, driver::Drivers::Papago);
    
    loop {
        
    }
    
}
