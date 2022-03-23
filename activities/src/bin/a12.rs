// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Dimensions {
    height: f32,
    width: f32,
    depth: f32,
}

impl Dimensions {
    fn new(height: f32, width: f32, depth: f32) -> Self {
        Dimensions {
            height,
            width,
            depth,
        }
    }

    fn print(&self) {
        println!(
            "Height: {}\nWidth: {}\nDepth: {}",
            self.height, self.width, self.depth
        );
    }
}

enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn print(&self) {
        print!("Color: ");
        match &self {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue"),
        }
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f32,
    color: Color,
}

impl Box {
    fn new(dimensions: Dimensions, weight: f32, color: Color) -> Self {
        Box {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
        self.color.print();
    }
}

fn main() {
    let my_box = Box::new(Dimensions::new(5.0, 7.0, 1.0), 59.3, Color::Blue);
    my_box.print()
}
