use std::{
    fs::File,
    io::{BufRead, BufReader},
    str,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
enum Move {
    Win,
    Lose,
    Draw,
}

impl TryFrom<&str> for Move {
    type Error = ();

    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Move::Lose),
            "Y" => Ok(Move::Draw),
            "Z" => Ok(Move::Win),
            &_ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl TryInto<i32> for Play {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Play::Rock => Ok(1),
            Play::Paper => Ok(2),
            Play::Scissors => Ok(3),
        }
    }
}

impl TryFrom<&str> for Play {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            // "X" => Ok(Play::Rock),
            // "Y" => Ok(Play::Paper),
            // "Z" => Ok(Play::Scissors),
            &_ => Err(()),
        }
    }
}

impl Play {
    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
    fn win_over(self, b: Play) -> bool {
        return match self {
            Play::Rock => b == Play::Scissors,
            Play::Scissors => b == Play::Paper,
            Play::Paper => b == Play::Rock,
        };
    }
}

// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper,
// and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round
// was a draw, and 6 if you won).
// a is me, b is the opponent.
fn calculate_score(a: Play, b: Play) -> i32 {
    let av: i32 = a.try_into().unwrap();
    if a == b {
        // drawn
        return 3 + av;
    }
    if a.win_over(b) {
        // i won
        return 6 + av;
    }
    // i lost
    return av;
}

fn find_play(a: Play, m: Move) -> Play {
    // this could prob be better
    return match m {
        Move::Draw => a,
        Move::Win => match a {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        },
        Move::Lose => match a {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        },
    };
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut score: i32 = 0;

    for line in r.lines() {
        let line = match line {
            Ok(v) => v,
            Err(_) => break,
        };

        let (col1, col2) = line.split_once(" ").unwrap();
        let my_move: Move = Move::try_from(col2).unwrap();
        let their_play: Play = Play::try_from(col1).unwrap();
        let my_play: Play = find_play(their_play, my_move);
        score += calculate_score(my_play, their_play);
    }
    println!("score: {}", score)
}
