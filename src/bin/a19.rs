// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;


fn main() {
    let mut items = HashMap::new();
    items.insert("chair", 5);
    items.insert("bed", 3);
    items.insert("table", 2);
    items.insert("couch", 0);

    let mut total = 0;

    for (content, quant) in items.iter() {
        total = total + quant;
        let stock_count = if quant == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", quant)
        };
        println!("item = {:?}, stock = {:?}", content, stock_count);
    }
    println!("total stock = {:?}", total);
}
