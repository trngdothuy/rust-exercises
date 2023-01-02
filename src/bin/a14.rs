// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    name: String,
    age: i32,
    color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

// * Create and store at least 3 people in a vector

// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

fn main() {
    let people = vec![
        Person {
            name:"Anna".to_owned(),
            age: 7,
            color: "green".to_owned(),
        },
        Person {
            name:"Bella".to_owned(),
            age: 11,
            color: "red".to_owned(),
        },
        Person {
            name:"Conan".to_owned(),
            age: 12,
            color: "black".to_owned(),
        },
        ];
        for person in people {
            if person.age <= 10 {
                print(&person.name);
                print(&person.color);
            }
        }
}
