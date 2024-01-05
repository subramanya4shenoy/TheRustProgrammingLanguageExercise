use std::io;
use std::collections::HashMap;

fn main() {
    // ask user if they want to add info or retrieve info?
    let mut user_choice: String = String::new();
    //collect user input for entry
    let mut user_cmd: String = String::new();

    let mut db: HashMap<String, String> = HashMap::new(); 

    println!("Hello!");
    println!("What do you want to do today?");
    println!("1. Do you want to retrive data?");
    println!("2. Do you want to add data? ");

    io::stdin().read_line(&mut user_choice);

    match user_choice.as_str().trim() {
        // Convert String to &str for matching
        "1" => {
            println!("Please enter department name");
            println!("{:#?}", db);
        } // Match against string literals
        "2" => {
            println!("Please enter new entry");
            io::stdin()
                .read_line(&mut user_cmd)
                .expect("Enter valid texts");
            let (name, department, operation) = extract_info(user_cmd.trim());
            // println!("{},{},{}", name, department, operation);
            db.insert(name, department);
            println!("{:#?}", db);
            // update our db
        }
        _ => {
            println!("Enter a valid choice!");
        }
    }

}

// Function to extract info from string
fn extract_info(text: &str) -> (String, String, String) {
    // identify is it a add operation or remove operation
    let operation: String = extract_operation(text);
    // get the name, department out of the string
    let(name , department) = extract_name_department(text);

    (name, department, operation)
}


// function to extract operation from string
fn extract_operation(text: &str) -> String {
    
    let mut operation: String;

    if text.to_lowercase().starts_with("add") {
        operation = String::from("ADD");
    } else if text.to_lowercase().starts_with("remove") {
        operation = String::from("REMOVE");
    } else {
        panic!("Error!!!")
    } 
    operation
}

// function to extract name and department from the string
fn extract_name_department(text: &str) -> (String, String) {
    let words_in_lowercase = text.to_lowercase();
    let word: Vec<&str> = words_in_lowercase.split_whitespace().collect();

    match word.as_slice() {
        ["add", name @ .., "to", department ] | ["remove", name @ .., "from", department  ] =>( name.join(" ").to_string(), department.to_string()),
        _ => (String::from("No valid name found"), String::from("No valid department found"))
    }
}
