// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f32, String),
    Vip(f32, String),
    Standard(f32),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(15.00, "Ian".to_owned()),
        Ticket::Vip(12.32, "Bob".to_owned()),
        Ticket::Standard(2.15),
        Ticket::Standard(5.95),
    ];

    for ticket in &tickets {
        match ticket {
            Ticket::Backstage(price, name) => {
                println!("Backstage ticket for {} at £{}", name, price)
            }
            Ticket::Vip(price, name) => println!("VIP ticket for {} at £{}", name, price),
            Ticket::Standard(price) => println!("Standard ticket at £{}", price),
        }
    }
}
