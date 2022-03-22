// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Blue => print!("Blue"),
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow"),
    }
}

fn main() {
    let color = Color::Blue;
    print_color(color);
}
