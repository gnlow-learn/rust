#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of rectangle is {} square pixels.",
        &rect1.area() // Rectangle::area(&rect1)
    );
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle { length: 40, width: 10 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2))
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}