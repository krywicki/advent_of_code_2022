use std::io::BufRead;

use aoc;

///

fn main() -> aoc::Result<()> {
    // part 1
    let sum_priorities: usize = aoc::open_puzzle_input("day_03.txt")?
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .sum();

    println!("summed priorties: {}", sum_priorities);

    // part 2
    let sum_badges: usize = aoc::open_puzzle_input("day_03.txt")?
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .chunks(3)
        .map(parse_lines)
        .sum();

    println!("summed badges: {}", sum_badges);

    Ok(())
}

fn parse_line(line: String) -> usize {
    let (part_1, part_2) = line.split_at(line.len() / 2);

    let mut priorities: Vec<usize> = part_1
        .chars()
        .filter(|c_1| part_2.chars().find(|c_2| c_1 == c_2).is_some())
        //.inspect(|c| print!("char: {}, value: {}", c, *c as usize))
        .map(char_to_usize)
        //.inspect(|v| println!(", pri. value: {}", v))
        .collect();

    priorities.sort();
    priorities.dedup();
    priorities.iter().sum()
}

fn parse_lines(lines: &[String]) -> usize {
    let (line_1, line_2, line_3) = (&lines[0], &lines[1], &lines[2]);

    let mut badges: Vec<usize> = line_1
        .chars()
        .filter(|c_1| {
            line_2
                .chars()
                .find(|c_2| {
                    line_3
                        .chars()
                        .find(|c_3| (c_1 == c_2) && (c_2 == c_3))
                        .is_some()
                })
                .is_some()
        })
        .map(char_to_usize)
        .collect();

    badges.sort();
    badges.dedup();
    badges.iter().sum()
}

fn char_to_usize(c: char) -> usize {
    match c {
        'A'..='Z' => c as usize - 38,
        'a'..='z' => c as usize - 96,
        _ => panic!("unknown character: {}", c),
    }
}
