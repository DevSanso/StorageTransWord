use std::error::Error;
use std::io::{self,BufRead};


use super::View;

pub struct ErrorView<T>  where T : Error{
    msg :  T
}


impl<T> ErrorView<T> where T : Error{
    pub fn new<'a>(err : T) -> ErrorView<T> {
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
        io::stdin().lock().read_line(&mut String::new())?;
        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn is_more_run(&self) -> bool {
        false
    }
    fn next<'b>(&self) -> Option<Box<dyn View + 'b>>{
        None
    }
}


pub struct ErrorStringView<'a>{
    msg : &'a str
}

impl<'a> ErrorStringView<'a>{
    pub fn new (err : &'a str) -> ErrorStringView {
        ErrorStringView {msg : err}
    }
}


impl<'a> View for ErrorStringView<'a> {
    fn display(&self) -> io::Result<()> {
        println!("error message : {}",self.msg);
        print!("press click enter");

        Ok(())
    }
    fn input(&mut self) -> io::Result<()> {
        io::stdin().lock().read_line(&mut String::new())?;
        Ok(())
    }
    fn update(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn is_more_run(&self) -> bool {
        false
    }
    fn next<'b>(&self) -> Option<Box<dyn View + 'b>>{
        None
    }
}