use std::env;
use std::fs;

fn parse_input(raw_input: String) -> Vec<u32> {
    let input_lines = raw_input.lines();
    let mut calories = Vec::new();
    let mut current_calorie: u32 = 0;

    for line in input_lines {
        if line.trim().len() == 0 {
            calories.push(current_calorie);
            current_calorie = 0;
        } else {
            current_calorie += line.parse::<u32>().unwrap();
        }
    }
    if current_calorie > 0 {
        calories.push(current_calorie);
    }

    calories
}

fn part1(elf_calories: &[u32]) -> (usize, u32) {
    elf_calories
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, &val)| (index, val))
        .unwrap()
}

fn part2(elf_calories: &[u32]) -> u32 {
    let mut sorted_elf_calories = Vec::from(elf_calories);
    sorted_elf_calories.sort_by(|a, b| a.cmp(b));
    sorted_elf_calories.reverse();
    sorted_elf_calories.iter().take(3).sum::<u32>()
}

fn main() {
    let mut file_name: String = "input_small.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let calories = parse_input(fs::read_to_string(file_name).expect("Could not read input file"));
    let part1_ans = part1(&calories);
    let part2_ans = part2(&calories);
    print!("Part 1: {}\n", part1_ans.1);
    print!("Part 2: {}\n", part2_ans);
}
