use std::io::BufRead;
use std::ops;

use aoc;

///

fn main() -> aoc::Result<()> {
    // part 1

    // parse stacks
    let mut iter = aoc::open_puzzle_input("day_05.txt")?
        .lines()
        .map(Result::unwrap);

    let stacks_str: String = iter
        .by_ref()
        .take_while(|line| !line.chars().all(char::is_whitespace))
        .map(|line| format!("{}\n", line))
        //.chain(std::iter::once("\n".into()))
        .collect();

    // move stacks
    let mut stacks = SupplyStacks::from(stacks_str);

    iter.by_ref()
        .map(StackMove::from)
        .for_each(|stack_move| stacks.move_stack(&stack_move));

    print!("day 05: part 1 - ");
    for x in 0..stacks.len() {
        print!("{}", stacks[x].last().unwrap_or(&' '));
    }
    println!();

    Ok(())
}

struct SupplyStacks {
    supply_stacks: Vec<Vec<char>>,
}

impl SupplyStacks {
    pub fn move_stack(&mut self, stack_move: &StackMove) {
        let supply_stacks = &mut self.supply_stacks;

        for _ in 0..stack_move.amount {
            let value = supply_stacks[stack_move.from_col].pop().unwrap();
            supply_stacks[stack_move.to_col].push(value);
        }
    }

    pub fn len(&self) -> usize {
        self.supply_stacks.len()
    }
}

struct StackMove {
    pub amount: usize,
    pub from_col: usize,
    pub to_col: usize,
}

impl From<String> for StackMove {
    fn from(value: String) -> Self {
        let nums: Vec<usize> = value
            .split_whitespace()
            .filter(|x| x.chars().all(char::is_numeric))
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        StackMove {
            amount: nums[0],
            from_col: nums[1] - 1,
            to_col: nums[2] - 1,
        }
    }
}

impl From<String> for SupplyStacks {
    fn from(value: String) -> Self {
        let num_cols: usize = value.lines().last().unwrap().split_whitespace().count();

        let mut stacks: Vec<Vec<char>> = vec![vec![]; num_cols];
        value.lines().rev().skip(1).for_each(|line| {
            let mut col = 0;
            line.chars().skip(1).step_by(4).for_each(|stack_item| {
                if !stack_item.is_whitespace() {
                    stacks[col].push(stack_item);
                }
                col += 1;
            });
        });
        // .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        // .for_each(|row| {
        //     row.iter().enumerate().for_each(|(col, stack_item)| {
        //         stacks[col].push(stack_item.chars().nth(1).unwrap())
        //     });
        // });

        SupplyStacks {
            supply_stacks: stacks,
        }
    }
}

impl ops::Index<usize> for SupplyStacks {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.supply_stacks[index]
    }
}
