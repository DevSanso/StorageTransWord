
pub struct Stack<T> {
    arr : Vec<T>
}


impl<T> Stack<T>{
    pub fn new_first(v: T) -> Self {
        let mut s = Stack{arr : Vec::new()};
        s.push(v);
        s
    }

    pub fn push(&mut self,v : T) {
            self.arr.push(v);
    }
    pub fn pop(&mut self) -> Option<T>{
        self.arr.pop()
    }
}