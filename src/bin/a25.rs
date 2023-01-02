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


trait  Perimeter  {
    fn calculate_perimeter(&self) -> i32;
}

struct  Square {
    side: i32,
}
impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct  Triangle  {
    a: i32,
    b: i32,
    c: i32,
}
impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.a + self.b + self.c
    }
}

fn print_perimeter(thing: impl Perimeter) {
    let perimeter  = thing.calculate_perimeter();
    println!("perimeter = {:?}", perimeter)
} 
fn main() {
    let my_square = Square {
        side: 3,
    };
    print_perimeter(my_square);
    let my_triangle  = Triangle  {
        a: 1,
        b: 2,
        c: 3,
    };
    print_perimeter(my_triangle);
}
