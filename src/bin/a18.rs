// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
struct Customer {
    age: i32,
}
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
fn purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err("You need to be older than 21 y.o to complete this purchase.".to_owned())
    }
}
// * The Err variant should detail the reason why they cannot make a purchase

fn main() {
    let student = Customer { age: 21 };
    let check = purchase(&student);
    println!("{:?}", check);
}
