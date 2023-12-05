use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

fn solve(input: Vec<&str>) -> String {
    let almanac = Almanac::parse(input);

    let computed_values = almanac.compute();

    let result = computed_values.iter().min().expect("no computed values");
    
    result.to_string()
}

#[derive(Debug)]
struct Instruction {
    dest: isize,
    range: isize,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<isize>,
    steps: Vec<BTreeMap<isize, Instruction>>,
}

impl Almanac {
    fn compute(&self) -> Vec<isize> {
        self.seeds
            .iter()
            .map(|&seed| {
                self.steps
                    .iter()
                    .enumerate()
                    .fold(seed, |acc, (_, step)| {
                        if let Some((&source, instruction)) =
                            step.range((Unbounded, Included(acc))).last()
                        {
                            let result = if !(source..(source + instruction.range)).contains(&acc) {
                                acc
                            } else {
                                acc + instruction.dest - source
                            };
                            result
                        } else {
                            acc
                        }
                    })
            })
            .collect()
    }

    fn parse(input: Vec<&str>) -> Self {
        let mut iter = input.iter();

        let seeds = iter
            .next()
            .expect("seeds not found")
            .split_once(':')
            .expect("seeds format is incorrect")
            .1
            .split(' ')
            .filter_map(|s| s.parse::<isize>().ok())
            .collect();

        let mut steps: Vec<BTreeMap<isize, Instruction>> = Vec::new();

        let mut iter = iter.peekable();

        while let Some(_) = iter.peek() {
            while let Some(_) = iter
                .next_if(|l| l.is_empty() || l.chars().next().is_some_and(|c| c.is_alphabetic()))
            {
            }
            let mut step: BTreeMap<isize, Instruction> = BTreeMap::new();
            while let Some(line) = iter.next() {
                if line.is_empty() {
                    break;
                }
                let parts: Vec<isize> = line
                    .splitn(3, ' ')
                    .filter_map(|p| p.parse::<isize>().ok())
                    .collect();
                let dest = parts.get(0).copied().expect("destination not found");
                let source = parts.get(1).copied().expect("source not found");
                let range = parts.get(2).copied().expect("range not found");

                step.insert(source, Instruction { dest, range });
            }
            steps.push(step);
        }

        Self { seeds, steps }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn day1_part1() {
        let input: Vec<_> = include_str!("part1-test.data").lines().collect();
        assert_eq!(solve(input), "13");
    }
}
