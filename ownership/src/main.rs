fn longest<'a, 'b >(x: &'a str, y: &'b str){
    if x.len() > y.len() {
        println!("{x}")
    } else {
        println!("{y}")
    }
}

fn main() {
    let x = String::from("hello");
    {
        let y = String::from("hi");
        longest(&x, &y);
    }
}


