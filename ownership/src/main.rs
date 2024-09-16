fn main() {
    let message = String::from("안녕하세요");
    let word = first_word(&message);
    println!("{}", word);

}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' '{
            return &s[..i]
        }
    }
    &s[..]
}








