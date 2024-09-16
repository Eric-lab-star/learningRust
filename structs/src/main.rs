#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 12,
        height: 12,
    };

    let sq = Rectangle::square(10);
    dbg!(&sq);

    let small_rect = Rectangle {
        width: 10,
        height: 10,
    };

    if rect.can_hold(&small_rect) {
        println!("yes");
    } else {
        println!("no");
    }
}
