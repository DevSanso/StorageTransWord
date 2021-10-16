use std::fmt::{Display,Formatter};
use std::fmt;

use crate::db::table::Book;



pub struct BookList{
    arr : Vec<Book>
}

impl super::Component for BookList {}

impl BookList {
    pub fn new(arr : Vec<Book>) -> BookList {
        BookList {arr :arr}
    }
}

impl Display for BookList {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        write!(f,"book_list\n\n")?;
        writeln!(f,"print => {id}  :  {name} \n\n",id = "bookID",name = "bookName")?;
        let count = self.arr.len();

        for i in 0..count-1 {
            writeln!(f," {id}  :  {name} ",id = self.arr[i].book_id,name = self.arr[i].name)?;
        }


        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::db::table::Book;
    #[test]
    fn book_list_test() {
        let mut v = Vec::new();
        for _ in 0..3 {
            v.push(Book{book_id : 0,name : String::from("sdf")})
        }
        let l = super::BookList::new(v);
        println!("{}",l);
    }
}