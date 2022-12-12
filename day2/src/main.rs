use std::env;
use std::fs;

enum RoundResult {
    Win,
    Draw,
    Loss,
}

fn parse_input(raw_input: String) -> Vec<(char, char)> {
    raw_input
        .lines()
        .map(|line| {
            let mut components = line.split_whitespace().take(2);
            (
                components.next().unwrap().chars().next().unwrap(),
                components.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect()
}

fn round_result(opponent: &char, you: &char) -> RoundResult {
    match (opponent, you) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => RoundResult::Draw,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => RoundResult::Win,
        _ => RoundResult::Loss,
    }
}

fn round_score(round: &(char, char)) -> u32 {
    let result_score = match round_result(&round.0, &round.1) {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Loss => 0,
    };

    let choice_score = (round.1 as u32) - ('X' as u32) + 1;
    result_score + choice_score
}

fn part2_strat(round: &(char, char)) -> (char, char) {
    let opponent = round.0;
    let outcome = round.1;
    let you = match outcome {
        // Loss
        'X' => match opponent {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => panic!("oops"),
        },

        // Draw
        'Y' => ((opponent as u8) - b'A' + b'X') as char,

        // Win
        'Z' => match opponent {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => panic!("oops"),
        },
        _ => panic!("oops"),
    };
    (opponent, you)
}

fn part1(rounds: &[(char, char)]) -> u32 {
    rounds.iter().map(round_score).sum::<u32>()
}

fn part2(rounds: &[(char, char)]) -> u32 {
    let new_rounds: Vec<(char, char)> = rounds.iter().map(part2_strat).collect();
    part1(&new_rounds)
}

fn main() {
    let mut file_name: String = "input_small.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let rounds = parse_input(fs::read_to_string(file_name).expect("Could not read input file"));
    let part1_score = part1(&rounds);
    let part2_score = part2(&rounds);
    println!("Part 1: {}", part1_score);
    println!("Part 2: {}", part2_score);
}
