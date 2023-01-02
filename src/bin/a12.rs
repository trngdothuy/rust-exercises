// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Dimension {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimension {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
}
}

// * Use an enum for the box color
enum Color {
    Red,
    Black,
    White,
}
impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Color: Red"),
            Color::Black => println!("Color: Black"),
            Color::White => println!("Color: White"),
    }
}
}
struct Box {
    dimension: Dimension,
    color: Color,
    weight: f64,
}
impl Box {
    fn new(weight: f64, color: Color, dimension: Dimension) -> Self {
        Self {
            weight,
            color,
            dimension,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimension.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let box_dimension = Dimension {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let my_box = Box::new(5.0, Color::Black, box_dimension);
    my_box.print();
}