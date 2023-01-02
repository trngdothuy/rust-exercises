// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
enum Flavor {
    Chocolate,
    Vanilla,
    Matcha,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid: f64,
}
// * Use a function to print out the drink flavor and ounces
fn name(drink: Drink) {
    match drink.flavor {
        Flavor::Chocolate => println!("Flavor: Chocolate"),
        Flavor::Vanilla => println!("Flavor: Vanilla"),
        Flavor::Matcha => println!("Flavor: Matcha"),
    }
    println!("Fluid oz: {:?}", drink.fluid);
}    
// * Use a match expression to print the drink flavor

fn main() {
    let chocolate = Drink {
        flavor: Flavor::Chocolate,
        fluid: 0.5
    };
    name(chocolate);
    let vanilla = Drink {
        flavor: Flavor::Vanilla,
        fluid: 1.0
    };
    name(vanilla);
    let matcha = Drink {
        flavor: Flavor::Matcha,
        fluid: 1.5
    };
    name(matcha);
}