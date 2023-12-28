use std::io;

fn main() {
    println!("Lets convert Temperatures");
    
    let mut value: String = String::new();
    let mut temp_type: String = String::new();
    
    println!("Enter the temperature: ");
    
    io::stdin()
        .read_line(&mut value)
        .expect("not a valid input");

    println!("you want to conver the temperature into 1. Celcius or 2. Farn8 ?");

    io::stdin()
        .read_line(&mut temp_type)
        .expect("not a valid input");

    let value: f64 = value
                    .trim()
                    .parse()
                    .expect("not a valid number");

    let temp_type: i32 = temp_type
                         .trim()
                         .parse()
                         .expect("not a valid number");
    
    if temp_type == 1 {
        let celcius: f64 = (5.0/9.0) * (value - 32.0);
        println!("Value in celcius is {celcius}");
    } else {
        let faran8: f64 = (9.0 / 5.0) * value + 32.0;
        println!("Value in Faran8 is {faran8}");
    }
}
