// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Locker {
    name: String,
    assignment: Option<i32>,
}


// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

fn main() {
    let ans = Locker {
        name: "Anna".to_owned(),
        assignment: Some(3),
    };

    println!("student name: {:?}", ans.name);

    match ans.assignment {
        Some(no) => println!{"assignment number: {:?}", no},
        None => println!{"no assignment number"}
    }
}
