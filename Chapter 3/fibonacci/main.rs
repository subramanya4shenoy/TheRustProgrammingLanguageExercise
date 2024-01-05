use std::io;

const WELCOME_MSG:&str = "Welcome to fib genrator";
const INSTRUCTION_MSG:&str = "Enter the number of Fib numbers required";

fn main() {
    println!("{}", WELCOME_MSG);
    println!("{}", INSTRUCTION_MSG);

    let mut count_str: String = String::new();
    
    io::stdin()
        .read_line(&mut  count_str)
        .expect("Not a valid number");

    let count_num: usize = match count_str.trim().parse() {
        Ok(num) => { num },
        Err(_) => { 
            println!("Please enter a valid number");
            return;
        }
    };

    fibonacci_number_gen(count_num);

}


fn fibonacci_number_gen(num:usize) {
    match num {
        0 => return,
        1 => println!("1"),
        _ => {
            let mut prev: usize = 0;
            let mut current: usize = 1;
            for _ in 1..=num {
                println!("{current}");
                let new = prev + current;
                prev = current;
                current = new;
            }
        }   
    }
}% 
