// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
// * Use a function to display the result
fn display(result: i32) {
    println!("{:?}", result);
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let result = sum(2, 2);
    display(result);
}
