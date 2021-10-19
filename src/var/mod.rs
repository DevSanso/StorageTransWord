use std::path::Path;
use std::include_str;
use std::fs::read_to_string;

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


static mut f : Option<VarField> = None;
const DBNAME : &str = "words.db";

impl Var {
    #[cfg(test)]
    pub fn test_init() -> Result<(),&'static str> {
        unsafe {
            if f.is_some() {
                return Err("already init var");
            }
        }
        let driver_config = String::from(include_str!("../../test_config/papago.json"));
        let conn = Connection::open_in_memory().unwrap();
        let trans_driver = driver::facory(driver_config, driver::Drivers::Papago);
        init_db(&conn);
        unsafe {
            f = Some(VarField {conn : conn,driver : trans_driver});
        }

        Ok(())
    }

    pub fn init() -> Result<(),&'static str> {
        unsafe {
            if f.is_some() {
                return Err("already init var");
            }
        }
        let db_path = Path::new("./").join(DBNAME);
        let driver_config = read_config();
        let conn = Connection::open(db_path).unwrap();
        let trans_driver = driver::facory(driver_config, driver::Drivers::Papago);
        init_db(&conn);
        unsafe {
            f = Some(VarField {conn : conn,driver : trans_driver});
        }

        Ok(())
    }
    pub  fn get_db_conn_as_mut_ref() -> &'static rusqlite::Connection{
        unsafe{    
            match &f {
                    Some(s) => &s.conn,
                    None => panic!("not init var")
            }
        }
    }

    pub fn get_driver_as_ref() ->&'static Box<dyn Driver> {
        unsafe {
            match &f {
                Some(s) => &s.driver,
                None => panic!("not init var")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn var_init_test() -> Result<(),&'static str>{
        super::Var::test_init();
        let conn = super::Var::get_db_conn_as_mut_ref();
        let driver = super::Var::get_driver_as_ref();
        println!("{}",driver.name());
        Ok(())
    }
}






