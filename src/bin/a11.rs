// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number

struct Grocery {
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity (item: &Grocery) {
    println!("quantity: {:?}", item.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id (item: &Grocery) {
    println!("id number: {:?}", item.id);
}

fn main() {
    let item = Grocery {
        quantity: 10,
        id: 45678,
    };
    display_quantity(&item);
    display_id(&item);
}
