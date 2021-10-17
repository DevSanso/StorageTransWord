use std::error::Error;
use std::io::{self,Read};

use super::View;

pub struct ErrorView<T>  where T : Error{
    msg : T
}


impl<T> ErrorView<T> where T : Error{
    pub fn new(err : T) -> ErrorView<T> {
        ErrorView {msg : err}
    }
}


impl<T> View for ErrorView<T> where T : Error {
    fn display(&self) -> io::Result<()> {
        println!("error message : {}",self.msg);
        print!("press click enter");

        Ok(())
    }
    fn input(&mut self) -> io::Result<()> {
        io::stdin().lock().read_to_string(&mut String::new())?;
        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn is_more_run(&self) -> bool {
        false
    }
    fn next(&self) -> Option<Box<dyn View>>{
        None
    }
}