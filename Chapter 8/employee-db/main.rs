use std::io;

fn main() {
    //collect user input
    let mut user_cmd: String = String::new();

    io::stdin()
        .read_line(&mut user_cmd)
        .expect("Enter valid texts");

    let (name, department) = extract_name_department(user_cmd.trim().to_string());

    println!("{},{}", name, department);
}

fn extract_name_department(text: String) -> (String, String) {
    (String::from("Subu"), String::from("CS"))
}