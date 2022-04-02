// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesColor(Color);

#[derive(Debug)]
struct ShirtColor(Color);

#[derive(Debug)]
struct PantsColor(Color);

impl ShoesColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl ShirtColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}
impl PantsColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirt_color(color: ShirtColor) {
    println!("Shirt color is {:?}", color);
}

fn print_shoes_color(color: ShoesColor) {
    println!("Shoes color is {:?}", color);
}
fn print_pants_color(color: PantsColor) {
    println!("Pants color is {:?}", color);
}

fn main() {
    let shirt_color = ShirtColor::new(Color::Red);
    let shoes_color = ShoesColor::new(Color::Black);
    let pants_color = PantsColor::new(Color::Green);

    print_shirt_color(shirt_color);
    print_shoes_color(shoes_color);
    print_pants_color(pants_color);
}
