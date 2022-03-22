// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sweet,
    Sour,
    Spicy,
}

struct Drink {
    oz: f64,
    flavor: Flavor,
}

fn print_drink(drink: Drink) {
    println!("Oz: {}", drink.oz);
    match drink.flavor {
        Flavor::Sour => println!("sour"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Spicy => println!("Spicy"),
    }
}

fn main() {
    let lemon_juice = Drink {
        oz: 15.0,
        flavor: Flavor::Sour,
    };
    print_drink(lemon_juice);
}
