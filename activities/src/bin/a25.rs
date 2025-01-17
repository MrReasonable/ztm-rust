// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calc_perimeter(&self) -> i32;
}

struct Square {
    len: i32,
}

impl Perimeter for Square {
    fn calc_perimeter(&self) -> i32 {
        self.len * 4
    }
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> i32 {
        self.a + self.b + self.c
    }
}

fn print_perimeter(shape: impl Perimeter) {
    println!("{}", shape.calc_perimeter());
}

fn main() {
    print_perimeter(Square { len: 10 });
    print_perimeter(Triangle { a: 5, b: 20, c: 53 });
}
