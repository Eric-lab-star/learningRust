use std::io;

fn main() {
    let nums_data = [1, 2, 3, 4, 5];
    let mut input = String::new();
    println!("Input a number");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: usize = input.trim().parse().expect("input should be number");
    println!("{}", nums_data[input])
}
