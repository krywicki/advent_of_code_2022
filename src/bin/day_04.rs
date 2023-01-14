use std::io::BufRead;

use aoc;

///

fn main() -> aoc::Result<()> {
    // part 1
    let dup_work_count = aoc::open_puzzle_input("day_04.txt")?
        .lines()
        .map(Result::unwrap)
        .map(parse_pt_1)
        .filter(|x| *x == true)
        .count();

    println!("day 04: part 1 - {dup_work_count}");

    // part 2
    let overlap_work_count = aoc::open_puzzle_input("day_04.txt")?
        .lines()
        .map(Result::unwrap)
        .map(parse_pt_2)
        .filter(|x| *x == true)
        .count();

    println!("day 04: part 2 - {overlap_work_count}");

    Ok(())
}

fn parse_pt_1(line: String) -> bool {
    line.split(',')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunks| (parse_range(&chunks[0]), parse_range(&chunks[1])))
        .map(|assignments| {
            let ((a_1, b_1), (a_2, b_2)) = assignments;
            match ((a_1, b_1), (a_2, b_2)) {
                _ if (a_1 <= a_2) && (b_2 <= b_1) => true, // if group_2 assignment within group_1
                _ if (a_2 <= a_1) && (b_1 <= b_2) => true, // if group_1 assignment within group_2
                _ => false,
            }
        })
        .reduce(|a, b| (a == b) && (b == true))
        .unwrap()
}

fn parse_pt_2(line: String) -> bool {
    line.split(',')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunks| (parse_range(&chunks[0]), parse_range(&chunks[1])))
        .map(|assignments| {
            let ((a_1, b_1), (a_2, b_2)) = assignments;
            match ((a_1, b_1), (a_2, b_2)) {
                _ if (a_1 <= a_2) && (b_1 >= a_2) => true, // overlap
                _ if (a_2 <= a_1) && (b_2 >= a_1) => true, // overlap
                _ => false,
            }
        })
        .reduce(|a, b| (a == b) && (b == true))
        .unwrap()
}
//fn parse_line()

fn parse_range(num_range: &str) -> (i32, i32) {
    num_range
        .split('-')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunks| {
            (
                chunks[0].parse::<i32>().unwrap(),
                chunks[1].parse::<i32>().unwrap(),
            )
        })
        .take(1)
        .collect::<Vec<(i32, i32)>>()[0]
}
