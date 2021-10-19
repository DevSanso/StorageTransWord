use std::io;
use std::process::Command;
use std::process::ExitStatus;


#[cfg(target_os = "linux")]
fn clean() -> io::Result<ExitStatus>{
    Command::new("clear").status()
}
#[cfg(target_os = "windows")]
fn clean() -> io::Result<ExitStatus> {
    Command::new("cls").status()
}




pub trait View {

    fn display(&self) -> io::Result<()> ;
    fn input(&mut self) -> io::Result<()>;
    fn update(&mut self) -> io::Result<()>;
    fn is_more_run(&self) -> bool;
    fn next<'parent>(&self) -> Option<Box<dyn View + 'parent>>;
    
    fn clear(&self) -> io::Result<()> {
        match clean() {
            Ok(ok) => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn exec(&mut self) -> io::Result<()> {
        self.clear()?;
        self.display()?;
        self.input()?;
        self.update()?;

        Ok(())
    }


}

pub mod main;
pub mod err;
pub mod make_book;

pub fn first_view<'a>() -> Box<dyn View + 'a> {
    Box::new(main::MainMenu::new())
}

#[cfg(test)]
mod tests {
    use std::io::{self,BufRead};
    #[test]
    fn clean_test() {
        println!("hello! \n");
        super::clean().unwrap();
    }
}