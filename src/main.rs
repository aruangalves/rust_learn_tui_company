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

        //splitting user input into a Vector
        let split_input: Vec<String> = user_input.split(' ').map(|s| s.to_string()).collect();

        //guarantees that somehow the input wasn't empty
        //so I can safely retrive the first element
        if split_input.is_empty() {
            println!("Invalid input, please try again.");
            continue;
        }

        let first_input: &String = split_input.first().unwrap();
        let first_input = first_input.clone().to_lowercase();

        //Test ok, first word is lowercased...
        //println!("The first input is {}", first_input);

        match first_input.as_str() {
            "add" => {
                println!("TODO: add new employee to hashmap...");
            }
            "list" => {
                println!("TODO: list employees by department...");
            }
            "exit" => break,
            _ => {
                println!("Invalid input, please try again.");
            }
        }

        println!("The word count is {}", split_input.len());

        for s in &split_input {
            println!("{}", s);
        }
    }
}
