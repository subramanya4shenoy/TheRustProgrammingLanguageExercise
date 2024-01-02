use std::io;

fn main() {

    let mut text = String::new();

    println!("Enter a text");

    io::stdin()
        .read_line(&mut text)
        .expect("not a valid string");
    
    println!("{}", pig_latin(&text.trim().to_string()));

}

fn pig_latin(t: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let pig_latin_words: Vec<String> = t.split_whitespace().map(|word| {
        if let Some(first_char) = word.chars().next() {
            if vowels.contains(&first_char.to_ascii_lowercase()) {
                // Word starts with a vowel
                format!("{}-hay", word)
            } else {
                // Word starts with a consonant
                format!("{}-{}ay", &word[1..], first_char)
            }
        } else {
            word.to_string()
        }
    }).collect();

    pig_latin_words.join(" ")
}
