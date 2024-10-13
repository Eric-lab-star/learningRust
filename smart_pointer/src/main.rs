use core::fmt;
use std::fmt::{Display, Formatter};

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut msg: Vec<i32> = vec![];
        List::walk(self, &mut msg);
        write!(f, "{:?}", msg)
    }
}

impl List {
    fn walk(list: &List, cup: &mut Vec<i32>){
        match list {
            Cons(v, b) => {
                cup.push(*v);
                List::walk(&b, cup);
            }
            _ => {}
        } 
    }

}


fn main() {
    let a_list = Cons(1, Box::new(Nil));
    let b_list = Cons(2, Box::new(a_list));
    let c_list = Cons(3, Box::new(b_list));

    println!("{}", c_list);
}

