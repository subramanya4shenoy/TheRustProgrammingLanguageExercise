fn main() {

    let s1: String = String::from("Hello World");
    let first_word: &str = get_first_word(&s1);
    println!("{first_word}");
}

fn get_first_word(s: &str) -> &str {
    println!("Inside the function {s}");
    &s[0..3]
}