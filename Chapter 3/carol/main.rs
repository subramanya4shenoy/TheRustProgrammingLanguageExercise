const DEFAULT_START: &str = "On the ";
const DEFAULT_END: &str = " day of Christmas my true love sent to me:";

const GIFTS: [&str; 12] = [
    "a Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "Five Golden Rings",
    "Six Geese a Laying",
    "Seven Swans a Swimming",
    "Eight Maids a Milking",
    "Nine Ladies Dancing",
    "Ten Lords a Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming",
];

fn main() {
    for i in 1usize..=12 {
        println!("{}{}{}", DEFAULT_START, ordinal(i), DEFAULT_END);
        for j in (1usize..=i).rev() {
            if i != 1 && j == 1 {
                print!("And ");
            }
            println!("{}", GIFTS[j - 1]);
        }
        println!(); // Adds an empty line for better readability
    }
}

fn ordinal(num: usize) -> String {
    let suffix = match num {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", num, suffix)
}
