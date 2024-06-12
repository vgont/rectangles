use std::io::{self, Write};

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
}

fn main() {
    println!("\nInsert the rectangle values\n");

    print!("Width(cm): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let width: u32 = input.trim().parse().unwrap();

    print!("Height(cm): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let height: u32 = input.trim().parse().unwrap();

    let rect = dbg!(Rectangle { width, height });

    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The rect has w:{}cm, h:{}cm. Its area is: {}cm",
        rect.width,
        rect.height,
        rect.area()
    );

    println!("The rect has a nonzero width: {}", rect.is_width_nonzero());
    println!("Your rect fits a {:?}: {}", rect2, rect.can_hold(&rect2));
    println!("Your rect fits a {:?}: {}", rect3, rect.can_hold(&rect3));
}
