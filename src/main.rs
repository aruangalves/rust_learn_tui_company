use std::io;
use std::io::Write;

fn main() {
    let mut user_input = String::new();

    loop {
        println!("Type an option: add employee | list department | exit");
        println!("Usage: ");
        println!("  Add John Doe to Engineering");
        println!("  List Engineering");
        println!("  Exit");

        io::stdout().flush().expect("Failed to flush");

        //Clear the string to not append last input
        user_input.clear();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
    }
}
