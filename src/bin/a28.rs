// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor{
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor{
    pub  fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple => Err("purple not allowed".to_owned()),
            other => Ok (Self(other)),
        }
    }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor{
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

fn  print_shirt_color(color: Result<ShirtColor, String>) {
    println!("Shirt color: {:?}", color);
}

fn  print_shoes_color(color: ShoesColor) {
    println!("Shoes color: {:?}", color);
}

fn  print_pants_color(color: PantsColor) {
    println!("Pants color: {:?}", color);
}


fn main() {
   let shirt_color = ShirtColor::new(Color::Red);
   let shoes_color = ShoesColor::new(Color::Black);
   let pants_color = PantsColor::new(Color::Brown);

   print_shirt_color(shirt_color);
   print_shoes_color(shoes_color);
   print_pants_color(pants_color);

}



