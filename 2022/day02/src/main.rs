use std::fs;

#[derive(PartialEq, Eq, Clone)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

impl Game {
    fn score(&self) -> i32 {
        return match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };
    }
}

#[derive(PartialEq, Eq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn score(&self) -> i32 {
        return match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        };
    }
}

use self::GameResult::{Draw, Lose, Win};

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read line.");

    // Part 1
    let score_list: Vec<i32> = data
        .lines()
        .map(|x| {
            let game_line: Vec<&str> = x.split(' ').collect();
            return calculate_score(
                parse_game_choice(game_line[0]),
                parse_game_choice(game_line[1]),
            );
        })
        .collect();
    println!(
        "Part 1. The total score is: {}",
        score_list.iter().sum::<i32>()
    );

    // Part 2
    let score_list: Vec<i32> = data
        .lines()
        .map(|x| {
            let game_line = x.split(' ').collect::<Vec<&str>>();
            let opponent = parse_game_choice(game_line[0]);
            return calculate_score(
                opponent.clone(),
                guess_game_shape(opponent.clone(), parse_game_guide(game_line[1])),
            );
        })
        .collect();
    println!(
        "Part 2. The total score is: {}",
        score_list.iter().sum::<i32>()
    );
}

// ------- Part 1 -------

fn parse_game_choice(text: &str) -> Game {
    // print!("{}, ", text);
    return match text {
        "A" | "X" => Game::Rock,
        "B" | "Y" => Game::Paper,
        "C" | "Z" => Game::Scissors,
        _ => {
            panic!("Failed to parse the game.");
        }
    };
}

fn calculate_score(a: Game, b: Game) -> i32 {
    return get_game_result(&a, &b).score() + b.score();
}

// Returns player B's game result
fn get_game_result(a: &Game, b: &Game) -> GameResult {
    if a == b {
        return Draw;
    } else if a == &Game::Rock && b == &Game::Paper {
        return Win;
    } else if a == &Game::Paper && b == &Game::Scissors {
        return Win;
    } else if a == &Game::Scissors && b == &Game::Rock {
        return Win;
    } else {
        return Lose;
    }
}

// ------- Part 2 -------

fn guess_game_shape(opponent: Game, expected_result: GameResult) -> Game {
    return match expected_result {
        Draw => opponent.clone(),
        Win => match opponent {
            Game::Rock => Game::Paper,
            Game::Paper => Game::Scissors,
            Game::Scissors => Game::Rock,
        },
        Lose => match opponent {
            Game::Rock => Game::Scissors,
            Game::Paper => Game::Rock,
            Game::Scissors => Game::Paper,
        },
    };
}

fn parse_game_guide(text: &str) -> GameResult {
    return match text {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!("Failed to parse the game strategy guide."),
    };
}
