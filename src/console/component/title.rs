use std::fmt;
use std::fmt::*;





pub struct Title {}

impl Title {
    pub fn new() -> Title {Title{}}
}

impl super::Component for Title {}

impl Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"process : {}\n",env!("CARGO_PKG_NAME"))?;
        write!(f,"author : {}",env!("CARGO_PKG_AUTHORS"))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn menu_test() {

        let m = super::Title::new();
        println!("{}",m);
    }
}