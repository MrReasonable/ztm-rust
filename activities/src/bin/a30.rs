// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

#[derive(Debug)]

struct Car;
#[derive(Debug)]
struct Truck;
#[derive(Debug)]
struct Scooter;

impl Body for Car {}
impl Body for Truck {}
impl Body for Scooter {}

#[derive(Debug)]
struct Red;
#[derive(Debug)]
struct Blue;
#[derive(Debug)]
struct Green;
impl Color for Red {}
impl Color for Blue {}
impl Color for Green {}

impl<B: Body, C: Color> Vehicle<B, C> {
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

fn main() {
    let car = Vehicle::new(Car, Red);
    let truck = Vehicle::new(Truck, Green);
    println!("{:?}", car);
    println!("{:?}", truck);
}
