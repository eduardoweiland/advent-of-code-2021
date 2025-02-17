use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let mut parsed_input = Input {
        rules: vec![],
        updates: vec![],
    };

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let pages: Vec<_> = line
            .split('|')
            .map(|page| str::parse(page).unwrap())
            .collect();
        parsed_input.rules.push((pages[0], pages[1]));
    }

    while let Some(line) = lines.next() {
        let pages: Vec<_> = line
            .split(',')
            .map(|page| str::parse(page).unwrap())
            .collect();
        parsed_input.updates.push(pages);
    }

    parsed_input
}

#[aoc(day5, part1)]
fn solve_part1(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| is_correctly_ordered(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
fn solve_part2(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| !is_correctly_ordered(update, &input.rules))
        .cloned()
        .map(|mut update| {
            update.sort_by(|a, b| {
                if input.rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if input.rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            update[update.len() / 2]
        })
        .sum()
}

fn is_correctly_ordered(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for rule in rules {
        let x_index = update.iter().position(|el| *el == rule.0);
        let y_index = update.iter().position(|el| *el == rule.1);

        if let (Some(x), Some(y)) = (x_index, y_index) {
            if x > y {
                return false;
            }
        }
    }

    true
}

#[derive(Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn it_solves_part1() {
        let answer = solve_part1(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 143);
    }

    #[test]
    fn it_solves_part2() {
        let answer = solve_part2(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 123);
    }
}
