use std::fs;
use std::time::Instant;

const FILENAME: &str = "input.txt";

const COLUMNS: usize = 9;

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

    let mut lines = input.lines();

    // Stacks of boxes (part 1 and 2)
    let mut stacks1: [Vec<char>; COLUMNS] = Default::default();
    let mut stacks2: [Vec<char>; COLUMNS];

    // Read the boxes
    'box_reading: loop {
        let line = lines.next();
        let mut chars = match line {
            Some(line) => line.chars(),
            None => break,
        };

        let mut stack = 0usize;
        loop {
            match chars.next() {
                Some(' ') => {
                    chars.next();
                    chars.next();
                    chars.next();
                }
                Some('[') => {
                    stacks1[stack].insert(0, chars.next().unwrap());
                    chars.next();
                    chars.next();
                }
                _ if line.unwrap().len() < 3 => break 'box_reading,
                _ => break,
            }
            stack += 1;
        };
    }
    stacks2 = stacks1.clone();

    // Move them
    loop {
        let line = lines.next();
        let mut chars = match line {
            Some(line) => line.chars().peekable(),
            None => break,
        };

        // Check for EOF
        let quantity_option = next_number(& mut chars);
        if quantity_option.is_none() { break; }

        let quantity = quantity_option.unwrap();
        let from = next_number(& mut chars).unwrap() as usize - 1;
        let to = next_number(& mut chars).unwrap() as usize - 1;

        for _ in 0..quantity {
            let crane_hand = stacks1[from].pop().unwrap();
            stacks1[to].push(crane_hand);
        }

        let mut crane_hand = Vec::<char>::new();
        for _ in 0..quantity {
            crane_hand.push(stacks2[from].pop().unwrap());
        }
        for _ in 0..quantity {
            stacks2[to].push(crane_hand.pop().unwrap());
        }
    }

    print!("Top of each stack (Part 1): ");
    for stack in stacks1 {
        print!("{}", stack[stack.len() - 1])
    }
    println!();

    print!("Top of each stack (Part 2): ");
    for stack in stacks2 {
        print!("{}", stack[stack.len() - 1])
    }
    println!();

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