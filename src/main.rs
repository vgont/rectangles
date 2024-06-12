#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn is_width_nonzero(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
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
        width: 20,
        height: 25,
    };
    dbg!(&rect);

    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };

    let square_rect = Rectangle::square(10);
    dbg!(&square_rect);

    println!("The rect area is: {}cm", rect.area());
    println!("The rect has a nonzero width: {}", rect.is_width_nonzero());
    println!("Rect fits a {:?}: {}", rect2, rect.can_hold(&rect2));
    println!("Rect fits a {:?}: {}", rect3, rect.can_hold(&rect3));
}
