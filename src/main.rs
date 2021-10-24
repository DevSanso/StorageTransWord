mod driver;
mod db;
mod console;
mod var;
mod stack;
mod help_str;
use stack::Stack;
use std::env;


fn main() -> std::io::Result<()> { 

    let hflag = env::args().find(|x|  x== "help" || x == "-h");
    if hflag.is_some() {
        println!("{}",help_str::HELP_USAGE);
        return Ok(());
    }

    let var_init_err = var::Var::init().err();
    if var_init_err.is_some() {
        panic!("{}",var_init_err.unwrap());
    }

    let mut s  = Stack::new_first(console::view::first_view());
    loop {
        let box_v = s.pop();
        if box_v.is_none() {break;}

        let mut v = box_v.unwrap();
        v.exec()?;
        let n = v.next();
        if v.is_more_run() {s.push(v);}
        if n.is_some() { s.push(n.unwrap());}
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::console;
    use crate::var;
    use crate::stack::Stack;

    #[test]
    fn main_test() -> std::io::Result<()> {
        let var_init_err = var::Var::test_init().err();
        if var_init_err.is_some() {
            panic!("{}",var_init_err.unwrap());
        }
        let mut s  = Stack::new_first(console::view::first_view());
        loop {
            let box_v = s.pop();
            if box_v.is_none() {break;}
    
            let mut v = box_v.unwrap();
            v.exec()?;
            let n = v.next();
            if v.is_more_run() {s.push(v);}
            if n.is_some() { s.push(n.unwrap());}
        }
        Ok(())
    }

}
