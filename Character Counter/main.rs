use std::io;
use std::collections::HashMap;


fn main() {
    println!("Enter a sentence");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let test_sentence : String = input.trim().to_lowercase();

    let mut letter_map:HashMap<char, u8> = HashMap::new();
    
    for letter in test_sentence.chars() {
        if letter == ' ' {
            continue;
        }
        
        let mut count: u8 = match letter_map.get(&letter) {
            Some(value) => *value,
            None => u8::MIN
        };
        
        count += 1;
        
        letter_map.insert(letter, count);
    }
    
    for (key, value) in &letter_map {
        println!("{}: {}", key, value);
    }
}