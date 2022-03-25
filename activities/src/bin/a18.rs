// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: u8,
}

fn can_purchase(cust: &Customer) -> Result<(), String> {
    if cust.age < 21 {
        Err("Customer is below the age of 21".to_owned())
    } else {
        Ok(())
    }
}

fn main() {
    let cust = Customer { age: 21 };
    match can_purchase(&cust) {
        Err(err) => println!("{:?}", err),
        Ok(_) => println!("Go ahead!"),
    }
}
