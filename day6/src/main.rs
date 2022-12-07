use std::fs;
use std::time::Instant;

const FILENAME: &str = "input.txt";

fn main() {
    // Start stopwatch
    let start_time = Instant::now();

    // Read file to string
    let input = match fs::read_to_string(FILENAME) {
        Ok(content) => content,
        Err(_) => {
            println!("Couldn't open {}", FILENAME);
            return;
        }
    };

    let chars: Vec<char> = input.chars().collect();

    let mut start_packet_end = 0;
    for i in 0..(chars.len() - 3) {
        if check_unique_chars(&chars[i..i+4]) { 
            start_packet_end = i + 4;
            break; 
        }
    }

    let mut message_end = 0;
    for i in 0..(chars.len() - 13) {
        if check_unique_chars(&chars[i..i+14]) { 
            message_end = i + 14;
            break; 
        }
    }

    println!("Part 1: {}", start_packet_end);
    println!("Part 2: {}", message_end);

    println!(
        "Completed in: {} seconds",
        start_time.elapsed().as_secs_f32()
    );
}

fn check_unique_chars(data: &[char]) -> bool {
    for i in 1..data.len() {
        for j in 0..i {
            if data[i] == data[j] {
                return false;
            }
        }
    }
    true
}