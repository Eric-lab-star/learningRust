use generics::{Summary, Tweet};

fn main() {
    let t = Tweet::new(String::from("sisy"));
    t.summarize();
}

