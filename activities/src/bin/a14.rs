// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: u8,
    favourite_color: String,
}

fn print_it(the_string: &str) {
    println!("{:?}", the_string);
}

fn main() {
    let people = vec![
        Person {
            name: "Ian".to_owned(),
            age: 20,
            favourite_color: "red".to_owned(),
        },
        Person {
            name: "Bob".to_owned(),
            age: 25,
            favourite_color: "blue".to_owned(),
        },
        Person {
            name: "Jack".to_owned(),
            age: 12,
            favourite_color: "mauve".to_owned(),
        },
        Person {
            name: "Steve".to_owned(),
            age: 98,
            favourite_color: "orange".to_owned(),
        },
    ];

    for person in &people {
        if person.age < 50 {
            println!("Age: {}", person.age);
            print!("Name: ");
            print_it(&person.name);
            print!("Favourite color: ");
            print_it(&person.favourite_color);
        }
    }
}
