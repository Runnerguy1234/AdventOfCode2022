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

    //***** Part One *****//
    // Find the duplicates and sum up their priority
    let mut sum1: u32 = 0;
    for line in input.lines() {
        let compartment_size = line.len() / 2;
        let compartment_one = &line[0..compartment_size];
        let compartment_two = &line[compartment_size..];

        let mut duplicate = ' ';
        for c in compartment_one.chars() {
            if compartment_two.contains(c) {
                duplicate = c;
                break;
            }
        }

        sum1 += get_priority(duplicate);
    }

    //***** Part Two *****//
    // get each bag's content
    let mut lines = input.lines();
    let mut sum2: u32 = 0;
    loop {
        let bag1 = lines.next();
        let bag2 = lines.next();
        let bag3 = lines.next();

        if bag1.is_none() || bag2.is_none() || bag3.is_none() {
            break;
        }

        let bag1 = bag1.unwrap();
        let bag2 = bag2.unwrap();
        let bag3 = bag3.unwrap();

        let mut duplicate = ' ';
        for c in bag1.chars() {
            if bag2.contains(c) && bag3.contains(c) {
                duplicate = c;
                break;
            }
        }

        sum2 += get_priority(duplicate);
    }

    println!("Part 1 sum: {sum1}");
    println!("Part 2 sum: {sum2}");
    println!(
        "Completed in: {} seconds",
        start_time.elapsed().as_secs_f32()
    );
}

fn get_priority(c: char) -> u32 {
    // Code assumes char encoding keeps
    // a-z and A-Z alphabetically in order,
    // even if both sets aren't next to each other,
    // as is the case in ascii
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => {
            panic!("tried to find priority of non-letter");
        }
    }
}