use std::io;

enum UserOperations {
    Add,
    Remove
}

fn main() {
    // ask user if they want to add info or retrieve info?
    let mut user_choice: String = String::new();
    //collect user input for entry
    let mut user_cmd: String = String::new();

    println!("Hello!");
    println!("What do you want to do today?");
    println!("1. Do you want to retrive data?");
    println!("2. Do you want to add data? ");

    io::stdin().read_line(&mut user_choice);

    match user_choice.as_str().trim() {
        // Convert String to &str for matching
        "1" => {
            println!("Please enter department name");
        } // Match against string literals
        "2" => {
            println!("Please enter new entry");
            io::stdin()
                .read_line(&mut user_cmd)
                .expect("Enter valid texts");
            let (name, department) = extract_name_department(user_cmd.trim().to_string());
            println!("{},{}", name, department);
        }
        _ => {
            println!("Enter a valid choice!");
        }
    }

}

fn extract_name_department(text: String) -> (String, String) {
    (String::from("Subu"), String::from("CS"))
}
