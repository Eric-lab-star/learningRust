use std::cell::RefCell;


fn main() {
    let runtime_list = RefCell::new(vec![1,2,3]);
    let mut iter = runtime_list.borrow_mut();
    for item in iter.iter_mut() {
        *item += 1;
    }
    println!("{}", iter[0]);

}



