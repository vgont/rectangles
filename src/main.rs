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

    let rectangle = dbg!(Rectangle { width, height });
    let rectangle_area = dbg!(rectangle.area());

    println!(
        "The rectangle has w:{}cm, h:{}cm. Its area is: {}cm",
        rectangle.width, rectangle.height, rectangle_area
    );

    println!(
        "The rectangle has a nonzero width: {}",
        rectangle.is_width_nonzero()
    );
}
