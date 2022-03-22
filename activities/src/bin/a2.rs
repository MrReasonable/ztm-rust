// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(num: i32) {
    println!("{:?}", num);
}

fn main() {
    let result = add(1, 5);
    display_result(result);
}
