use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 50);
    scores.entry(String::from("blue")).or_insert(100);

    println!("{scores:#?}");

}
