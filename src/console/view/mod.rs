use std::io;
use std::process::Command;
use std::process::ExitStatus;


#[cfg(target_os = "linux")]
pub fn clean() -> io::Result<ExitStatus>{
    Command::new("clear").status()
}
#[cfg(target_os = "windows")]
pub fn clean() -> io::Result<ExitStatus> {
    Command::new("cls").status()
}



trait View {
    fn clear() -> io::Result<ExitStatus> {
        clean()
    }
}










#[cfg(test)]
mod tests {

    #[test]
    fn clean_test() {
        println!("hello! \n");
        super::clean().unwrap();
    }
}