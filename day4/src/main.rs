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

    let mut chars = input.chars().peekable();

    let mut num_containing: u32 = 0;
    let mut num_overlapping: u32 = 0;

    loop {
        let elf1_range = (next_number(&mut chars), next_number(&mut chars));
        let elf2_range = (next_number(&mut chars), next_number(&mut chars));
        if elf1_range.0.is_none()
            || elf1_range.1.is_none()
            || elf2_range.0.is_none()
            || elf2_range.1.is_none()
        {
            break;
        }

        let elf1_range = (elf1_range.0.unwrap(), elf1_range.1.unwrap());
        let elf2_range = (elf2_range.0.unwrap(), elf2_range.1.unwrap());

        if (elf1_range.0 <= elf2_range.0 && elf1_range.1 >= elf2_range.1)
            || (elf1_range.0 >= elf2_range.0 && elf1_range.1 <= elf2_range.1)
        {
            num_containing += 1;
        }

        if u32::max(elf1_range.0, elf2_range.0) <= u32::min(elf1_range.1, elf2_range.1) {
            num_overlapping += 1;
        }
    }

    println!("Part 1: {num_containing}");
    println!("Part 2: {num_overlapping}");

    println!(
        "Completed in: {} seconds",
        start_time.elapsed().as_secs_f32()
    );
}

fn next_number(data: &mut std::iter::Peekable<std::str::Chars>) -> Option<u32> {
    if data.peek().is_none() {
        return None;
    }

    // consume garbage
    loop {
        match data.peek() {
            Some(c) if c.is_numeric() => break,
            Some(_) => drop(data.next()),
            None => break,
        }
    };

    let mut num: Vec<char> = Vec::new();
    for c in data {
        if c.is_numeric() { num.push(c) }
        else { break; }
    };

    Some(String::from_iter(&num[..]).parse::<u32>().unwrap())
}