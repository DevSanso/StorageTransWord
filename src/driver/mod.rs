mod papago;

use papago::PapagoDriver;


pub trait Config  {
    fn new(source : String) -> Result<Self,String> where Self : Sized;
    fn getAttr(&self,name : &str) -> String;
}

pub enum Drivers {
    Papago
}


pub struct Response {
    pub status_code : u16,
    pub data : String
}



pub trait Driver  {
    fn trans(&self,word : String) -> Response;
    fn name(&self) -> &'static str;
}


pub fn facory(config : String,target : Drivers) -> Box<dyn Driver> {
    let c = match target {
        Drivers::Papago => papago::PapagoConfig::new(config),
    };
    Box::new(papago::PapagoDriver::new(c.unwrap()))
    
}






#[cfg(test)]
mod tests {
    use std::include_str;

    

    #[test]
    fn test_factory_marco() {
      
        
        let source = String::from(include_str!("../../test_config/papago.json"));
        let d = super::facory(source, super::Drivers::Papago);
        let res = d.trans(String::from("hello"));
        println!("{}",res.data);
    }
}
