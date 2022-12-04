use std::fs;
use std::time::Instant;

const FILENAME: &str = "input.txt";

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;

const LOSE_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, PartialEq)]
enum EndState {
    Lose,
    Draw,
    Win,
}

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

    //*****  Part One:   *****//
    // Turn string into iter
    let mut char_iter = input.chars();

    // Calculate scores from iter and accumulate them
    let mut cum_score1: i32 = 0;
    loop {
        let c1 = get_next_alphanumeric(&mut char_iter);
        let c2 = get_next_alphanumeric(&mut char_iter);

        if c1 == None || c2 == None {
            break;
        }

        let c1 = c1.unwrap();
        let c2 = c2.unwrap();

        let other_hand = Shape::from(c1);
        let my_hand = Shape::from(c2);

        cum_score1 += my_hand.score_against(other_hand);
    }

    //*****  Part Two:   *****//
    // Turn string into iter
    let mut char_iter = input.chars();

    let mut cum_score2: i32 = 0;
    loop {
        let c1 = get_next_alphanumeric(&mut char_iter);
        let c2 = get_next_alphanumeric(&mut char_iter);

        if c1 == None || c2 == None {
            break;
        }

        let c1 = c1.unwrap();
        let c2 = c2.unwrap();

        let other_hand = Shape::from(c1);
        let end_state = EndState::from(c2);

        let my_hand = end_state.calculate_hand(other_hand);

        cum_score2 += my_hand.score_against(other_hand);
    }

    println!("Your score for part 1 is {cum_score1}");
    println!("Your score for part 2 is {cum_score2}");
    println!(
        "Completed in: {} seconds",
        start_time.elapsed().as_secs_f32()
    );
}

fn get_next_alphanumeric(data: &mut std::str::Chars) -> Option<char> {
    loop {
        match data.next() {
            Some(c) if c.is_alphanumeric() => return Some(c),
            Some(_) => continue,
            None => return None,
        }
    }
}

impl Shape {
    pub fn beats(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn beaten_by(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn score_against(self, other: Shape) -> i32 {
        let hand_score: i32 = match self {
            Self::Rock => ROCK_SCORE,
            Self::Paper => PAPER_SCORE,
            Self::Scissors => SCISSORS_SCORE,
        };

        let win_score: i32 = if self == other {
            DRAW_SCORE
        } else if self.beats() == other {
            WIN_SCORE
        } else {
            LOSE_SCORE
        };

        hand_score + win_score
    }
}

impl EndState {
    fn calculate_hand(self, opponent: Shape) -> Shape {
        match self {
            Self::Lose => opponent.beats(),
            Self::Draw => opponent,
            Self::Win => opponent.beaten_by(),
        }
    }
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Shape::from<char> called on '{c}'"),
        }
    }
}

impl From<char> for EndState {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("EndState::from<char> called on '{c}'"),
        }
    }
}
