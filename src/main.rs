use std::io::{self, Write};

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
    println!("\nInsert the rectangle values\n");

    print!("Width: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let width: u32 = input.trim().parse().unwrap();

    print!("Height: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let height: u32 = input.trim().parse().unwrap();

    let rectangle = Rectangle { width, height };

    let rectangle_area = rectangle.area();

    println!(
        "The rectangle has w:{}cm, h:{}cm. Its area is: {}cm",
        rectangle.width, rectangle.height, rectangle_area
    )
}
