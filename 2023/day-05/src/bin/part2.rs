use std::collections::{HashMap, HashSet};

pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

fn solve(input: Vec<&str>) -> String {
    let mut count_map: HashMap<usize, usize> = HashMap::new();
    let result = input
        .iter()
        .filter_map(|l| l.split_once(":"))
        .filter_map(|(_, l)| l.split_once("|"))
        .enumerate()
        .map(|(id, (card, numbers))| {
            let card: HashSet<usize> = card
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect();
            let winning_numbers = numbers
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .filter(|n| card.contains(n))
                .count();
            let card_count = count_map.get(&id).copied().unwrap_or_default() + 1;
            ((id + 1)..=(id + winning_numbers)).for_each(|n| {
                let v = count_map.entry(n).or_default();
                *v += card_count;
            });
            card_count
        })
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn part2() {
        let input: Vec<_> = include_str!("part1-test.data").lines().collect();
        assert_eq!(solve(input), "30");
    }
}
