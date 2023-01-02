// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
use std::io;
// Notes:
// * Use an enum to store the possible power states
enum Keyword {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
impl Keyword {
    fn new(ans: &str) -> Option<Keyword> {
        let ans = ans.trim().to_lowercase();
        // String => &str 
        match ans.as_str() {
            "off" => Some(Keyword::Off),
            "sleep" => Some(Keyword::Sleep),
            "reboot" => Some(Keyword::Reboot),
            "shutdown" => Some(Keyword::Shutdown),
            "hibernate" => Some(Keyword::Hibernate),
            _ => None,
        }
    }   
}

fn print_ra(ans: Keyword) {
    // gọi lại keyword để ko phải nhắc lại ở từng dòng trong match, *  = all
    use Keyword::*;
    match ans {
    Off => println!("Turning off"),
    Sleep => println!("Sleeping"),
    Reboot => println!("Rebooting"),
    Shutdown => println!("Shutting down"),
    Hibernate => println!("Hibernating"),
    }
}


        


// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

fn main() {
    let mut ans = String::new(); 
    println!("Insert the power state.");
    let user_input = io::stdin().read_line(&mut ans);
    if user_input.is_ok() {
        match Keyword::new(&ans) {
            Some(ans) => print_ra(ans),
            None => println!("invalid power state"),
        } 
    } else {
        println!("error reading input");
    }
}
