mod driver;
mod db;
mod console;
mod var;



struct Stack<T> {
    arr : Vec<T>
}


impl<T> Stack<T>{
    fn new_first(v: T) -> Self {
        let s = Stack{arr : Vec::new()};
        s.push(v);
        s
    }

    fn push(&mut self,v : T) {
            self.arr.push(v);
    }
    fn pop(&mut self) -> Option<T>{
        self.arr.pop()
    }
}


fn main() -> std::io::Result<()> { 

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
