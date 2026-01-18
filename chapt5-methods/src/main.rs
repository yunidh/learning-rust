#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        if self.area() > other_rect.area() {
            true
        } else {
            false
        }
    }
    fn new_rect(width: u32, height: u32) -> Self {
        Self {
            // Same as writing `Rectangle {`
            width,
            height,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    println!(
        "Creating a new rect with constructor ::new_rect {:?}",
        Rectangle::new_rect(100, 50)
    )
}
