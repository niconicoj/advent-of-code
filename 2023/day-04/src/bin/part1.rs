use std::collections::HashSet;

pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

fn solve(input: Vec<&str>) -> String {
    let result = input
        .iter()
        .filter_map(|l| l.split_once(":"))
        .filter_map(|(_, l)| l.split_once("|"))
        .map(|(card, numbers)| {
            let card: HashSet<usize> = card
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect();
            let winning_numbers = numbers
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .filter(|n| card.contains(n))
                .count();
            match winning_numbers {
                0 => 0,
                n => 2usize.pow((n - 1) as u32),
            }
        })
        .reduce(|acc, v| acc + v)
        .unwrap();

    result.to_string()
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
