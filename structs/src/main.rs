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
        self.width > other_rect.width && self.height > other_rect.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect0 is {:#?}", rect0);

    println!("The area of the rect0 is {} square pixels.", rect0.area());

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("rect0 can hold rect1: {}", rect0.can_hold(&rect1));
    println!("rect0 can hold rect2: {}", rect0.can_hold(&rect2));

    let rect3 = Rectangle::square(25);

    println!("rect3 is {:#?}", rect3);
}
