// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
enum Color {
    Red, 
    White, 
    Black,
}
// * Use a function to print the color name
// * The function must use the enum as a parameter
fn name(my_color: Color) {
    match my_color {
        Color::Black => println!("black"),
        Color::Red => println!("red"),
        Color::White => println!("white"),
    }
}
// * Use a match expression to determine which color
//   name to print

fn main() {
    name(Color::Red);
}
