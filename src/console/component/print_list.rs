use std::fmt::{Display,Formatter};
use std::fmt;

use crate::db::table::{Book,Word};



pub struct BookList{
    arr : Vec<Book>
}

impl super::Component for BookList {}

impl BookList {
    pub fn new(arr : Vec<Book>) -> BookList {
        BookList {arr :arr}
    }
    pub fn search_book_id(&self,name : String) -> i32 {
        let mut iter = self.arr.iter();
        
        let f = iter.find(move |&x| x.name == name); 
        if f.is_none() {
            return -1;
        }
        return f.unwrap().book_id;
    }
}

impl Display for BookList {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        write!(f,"book_list\n\n")?;
        writeln!(f,"print => {id}  :  {name} \n\n",id = "bookID",name = "bookName")?;
        let count = self.arr.len();
        
        if count == 0 {
            return Ok(());
        }

        for i in 0..count  {
            writeln!(f," {id}  :  {name} ",id = self.arr[i].book_id,name = self.arr[i].name)?;
        }


        Ok(())
    }
}

pub struct WordList {
    book_name : String,
    chapter : i32,
    words : Vec<Word>
}

impl WordList {
    pub fn new(book_name : String,chapter : i32,words : Vec<Word>) -> WordList {
        WordList {book_name : book_name,chapter : chapter, words : words}
    }
}

impl Display for WordList {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        writeln!(f,"{:<12} => {:>24}","Book Name",self.book_name);
        writeln!(f,"{:<12}  : {:03} ","chapter",self.chapter);
        let word_count  = self.words.len();
        
        writeln!(f,"{ } => {}\n\n","Word Count",word_count);
        
        
        if word_count == 0 {
            return Ok(());
        }
        writeln!(f,
            "| {:>5} | {:>20} | {:>20}  |\n",
            "page","origin text","trans text"
        );
        for i in 0..word_count {
            let item = &self.words[i];
            writeln!(f,
                "| {:05} | {:>20} | {:>20} |",
            item.page,item.origin_text.replace("\n", ""),item.trans_text);
        }
        writeln!(f,"\n\n");

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::db::table::Book;
    use crate::db::table::Word;
    #[test]
    fn book_list_test() {
        let mut v = Vec::new();
        for i in 0..3 {
            let mut s = String::from("sdf");
           s.push_str(
                format!("{}",i).as_str()
            );
            v.push(Book{book_id : i,name : s})
        }
        let l = super::BookList::new(v);
        let a = l.search_book_id(String::from("sdf1"));
        println!("{}",l);
        println!("{}",a);
    }
    #[test]
    fn word_list_test() {
        let mut v = Vec::new();
        for i in 0..4 {
            let w = Word {
                book_id : 0,
                chapter : i,
                page : 0,
                origin_text : String::from("dhellof"),
                trans_text : String::from("dd")
            };
            v.push(w);
        }

        let l = super::WordList::new(String::from("hello"),1,v);
        println!("{}",l);
    }
}