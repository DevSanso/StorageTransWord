use std::path::Path;
use std::include_str;
use std::fs::read_to_string;
#[macro_use]
use lazy_static::lazy_static;

use rusqlite::Connection;

use crate::db;
use crate::db::init::init_db;
use crate::driver::{self,Driver};


pub struct Var {}

struct VarField {
    conn : Connection,
    driver : Box<dyn Driver>
}

fn read_config() -> String {
    let p = Path::new("./").join("./driver_config.json");
    read_to_string(p).unwrap()
}


lazy_static! {
    static ref f : VarField = {
       
        let db_path = Path::new("./").join(DBNAME);
        let driver_config = read_config();
        let conn = Connection::open(db_path);
        let trans_driver = driver::facory(driver_config, driver::Drivers::Papago);
        VarField {conn : conn.unwrap(),driver : trans_driver}
    };
}
const DBNAME : &str = "words.db";

impl Var {
  
    pub fn get_db_conn_as_mut_ref() -> &'static mut rusqlite::Connection{
        &mut f.conn
    }

    pub fn get_driver_as_ref() ->&'static Box<dyn Driver> {
        &f.driver
    }
}






