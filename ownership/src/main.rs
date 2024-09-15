fn main() {
    let dr = dangling_reference();
}

fn  dangling_reference() -> String{
    let s = String::from("hello");
    s
}






