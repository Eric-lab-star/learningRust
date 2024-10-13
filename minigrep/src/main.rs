use std::{rc::Rc};


fn main() {
    let mut v: Vec<Rc<String>> = vec![];
    { 
        let msg1 = Rc::new(String::from("hi"));
        println!("{}", Rc::strong_count(&msg1));
        v.push(Rc::clone(&msg1));
    }
    println!("v: {:?}", v);
    
}
