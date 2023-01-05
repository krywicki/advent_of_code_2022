use std::io::BufRead;

use aoc;

enum PuzzleInput {
    Calorie(usize),
    Break,
}

///

fn main() -> aoc::Result<()> {
    let reader = aoc::open_puzzle_input("day_01.txt")?;

    let mut total_calories = vec![0_usize];

    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| match line.parse::<usize>() {
            Ok(num) => PuzzleInput::Calorie(num),
            _ => PuzzleInput::Break,
        })
        .for_each(|p_input| match p_input {
            PuzzleInput::Break => total_calories.push(0),
            PuzzleInput::Calorie(num) => {
                let index = total_calories.len() - 1;
                total_calories[index] += num;
            }
        });

    total_calories.sort();
    total_calories.reverse();

    // day 1a
    println!("day 1a: {}", &total_calories[0]);

    // day 1b
    println!(
        "day 1b: {:#?}",
        total_calories.iter().take(3).sum::<usize>()
    );

    Ok(())
}
