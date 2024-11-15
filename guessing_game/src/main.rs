use std::io;

fn main() {
    println!("Guessing the number");
    println!("Please input your guess.");
    let mut guess = String::from("user input");
    io::stdin()
        .read_line(&mut guess) // references are immutable by default
        .expect("Failed to read line");
    println!("you guessed: {}", guess);
}
