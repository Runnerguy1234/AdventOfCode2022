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

    // Get peekable iterator of chars
    let mut chars = input.chars().peekable();

    let mut num_containing: u32 = 0;    // Part 1 result
    let mut num_overlapping: u32 = 0;   // Part 2 result

    loop {
        // Range of the first elf, before the comma
        let elf1_range = (next_number(&mut chars), next_number(&mut chars));
        // Range of the second elf, after the comma
        let elf2_range = (next_number(&mut chars), next_number(&mut chars));

        // If reached EOF, exit
        // (If the first one isn't None, the other's never will be
        // unless the file is invalid, (but it wont be(but it will break if it is (so be careful (stay safe (🦀)))))))))
        if elf1_range.0.is_none()
        {
            break;
        }

        // Unwrap the ranges because we know they're fine unless the file is invalid
        let elf1_range = (elf1_range.0.unwrap(), elf1_range.1.unwrap());
        let elf2_range = (elf2_range.0.unwrap(), elf2_range.1.unwrap());

        // Part One - if one range's boundaries are both outside of the others, increment a counter
        if (elf1_range.0 <= elf2_range.0 && elf1_range.1 >= elf2_range.1)
            || (elf1_range.0 >= elf2_range.0 && elf1_range.1 <= elf2_range.1)
        {
            num_containing += 1;
        }

        // Part Two - if there's any point of intersection, increment a counter
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
    // Make sure we're not already at the end of the file
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

    // get the characters of the number
    let mut num: Vec<char> = Vec::new();
    for c in data {
        if c.is_numeric() { num.push(c) }
        else { break; }
    };

    // Some( - there is a number
    // String::from_iter(&num[..]) - the slice is cast to an iter and collected to a string so it can be parsed
    // .parse::<u32>().unwrap()) - Convert to an int, then trust it enough to error if there wasn't actually a number
    Some(String::from_iter(&num[..]).parse::<u32>().unwrap())
}