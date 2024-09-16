
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}




fn main() {
    let rect = Rectangle{
        width: 12,
        height: 12,
    };
    println!("Area of rect is equal to {}", rect.area());
}




