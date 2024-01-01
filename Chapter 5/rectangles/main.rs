#[derive(Debug)]
struct Rectangle {
    width: u128,
    height: u128,
}

impl Rectangle {
    fn area(&self) -> u128 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn sqare(size: u128) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    
    let rect1: Rectangle = Rectangle {
        width: 40,
        height: 10,
    };

    let rect2: Rectangle = Rectangle {
        width: 2,
        height: 3,
    };

    let sq: Rectangle = Rectangle::sqare(20);
    
    println!("area = {} and rect = {:#?} can it hold 2? {}, and my square is {:#?}", rect1.area(), rect1, rect1.can_hold(&rect2), sq);
}

