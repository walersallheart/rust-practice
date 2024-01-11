use std::io;
use std::collections::HashMap;


fn main() {
    println!("Enter a sentence");
    let mut test_sentence = String::new();

    io::stdin()
        .read_line(&mut test_sentence)
        .expect("Failed to read line");

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