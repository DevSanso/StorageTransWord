use std::error::Error;

pub mod papago;


pub trait Config  {
    fn new(source : String) -> Result<Self,String> where Self : Sized;
    fn getAttr(&self,name : &str) -> String;
}




pub struct Response {
    statusCode : u16,
    data : String
}


pub trait Driver {
    fn new(config : impl Config) -> Self;
    fn trans(&self,word : String) -> Response;
}




