fn main() {
    let word = "안녕kim";

    let mut chars = word.chars();

    let count = chars.next();
    println!("{:?}", chars.next());
}
