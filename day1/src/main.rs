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

    // Array of each elf's total cals
    let mut sums: Vec<u32> = Vec::new();

    // Loop to go through each line, summing up each elf's calories
    let mut current_sum: u32 = 0;
    for line in input.lines() {
        match str::parse::<u32>(line) {
            // If there was a number...
            Ok(num) => {
                current_sum += num;
            }
            // If there was an empty line (couldn't be parsed as a number)
            Err(_) => {
                sums.push(current_sum);
                current_sum = 0;
            }
        }
    }

    // Rank every elf
    sums.sort();
    let elves = sums.len();

    println!("Part 1:");
    println!("  Top cal count: {}", sums[elves - 1]);
    println!();
    println!("Part 2:");
    println!("  Top cal counts: {:?}", &sums[elves - 3..elves - 0]);
    println!(
        "  Sum: {}",
        sums[elves - 1] + sums[elves - 2] + sums[elves - 3]
    );
    println!();
    println!(
        "Completed in: {} seconds",
        start_time.elapsed().as_secs_f32()
    );
}
