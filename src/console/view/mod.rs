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



trait View {

    fn display(&self) -> io::Result<()> ;
    fn input(&mut self) -> io::Result<()>;
    fn update(&mut self) -> io::Result<()>;
    fn is_running(&self) -> bool;
    fn next(&self) -> Option<Box<dyn View>>;
    
    fn clear(&self) -> io::Result<()> {
        match clean() {
            Ok(ok) => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn exec(&mut self) -> io::Result<()> {
        while self.is_running() {
            self.clear()?;
            self.display()?;
            self.input()?;
            self.update()?;
        }

        Ok(())
    }


}

pub mod main;





#[cfg(test)]
mod tests {

    #[test]
    fn clean_test() {
        println!("hello! \n");
        super::clean().unwrap();
    }
}