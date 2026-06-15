use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut depts: HashMap<String, Vec<String>> = HashMap::new();
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
        let split_input: Vec<String> = user_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        //guarantees that somehow the input wasn't empty
        //so I can safely retrive the first element
        if split_input.is_empty() {
            println!("Invalid input, please try again.");
            continue;
        }

        let first_input: &String = split_input.first().unwrap();
        let first_input = first_input.clone().to_lowercase();

        match first_input.as_str() {
            "add" => {
                add_employee(&split_input, &mut depts);
            }
            "list" => {
                list_by_dept(&split_input, &depts);
            }
            "exit" => break,
            _ => {
                println!("Invalid input, please try again.");
            }
        }
    }
}

fn add_employee(split_input: &[String], depts: &mut HashMap<String, Vec<String>>) {
    let mut employee = String::new();
    let mut dept = String::new();
    let mut is_valid = true;
    let mut employee_or_dept = true;

    for s in split_input.iter().skip(1) {
        if !is_valid {
            break;
        }

        match s.to_lowercase().as_str() {
            "to" => {
                if employee.is_empty() {
                    println!("Invalid input, you need to provide an employee name");
                    is_valid = false;
                    break;
                }
                employee_or_dept = false;
            }
            _ => {
                //add word to buffer...
                if employee_or_dept {
                    //Add to buffer employee...
                    employee.push_str(s);
                    employee.push(' ');
                } else {
                    //Add to buffer dept...
                    dept.push_str(s);
                    dept.push(' ');
                }
            }
        }
    }

    //Remove unecessary whitespaces...
    employee = String::from(employee.trim());
    dept = String::from(dept.trim());

    if !is_valid || dept.is_empty() || employee.is_empty() {
        println!("Invalid input, you need to provide an employee and department name.");
        return;
    }

    depts.entry(dept.clone()).or_default().push(employee);
    depts.entry(dept).or_default().sort();
}

fn list_by_dept(split_input: &[String], depts: &HashMap<String, Vec<String>>) {
    let dept = split_input[1..].join(" ");
    let dept = String::from(dept.trim());

    if dept.is_empty() {
        println!("Invalid input, you must typue a department name.");
        return;
    }

    if let Some(employees) = depts.get(&dept) {
        println!("These are the employees that work on {}:", dept);
        for e in employees {
            println!("{}", e);
        }
    }
}
