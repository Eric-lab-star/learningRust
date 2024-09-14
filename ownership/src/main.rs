fn main() {
    let mut s = String::from("Hello");
    s = moving_ownership(s);
    println!("{s}");
}

fn moving_ownership(mut s: String) -> String{
    s.push_str(", world");
    s
}
