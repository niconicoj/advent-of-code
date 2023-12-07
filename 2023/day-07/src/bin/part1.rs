use day_07::Hand;

include!("../codegen_p1.rs");

pub fn main() {
    let mut input: Vec<Hand> = include_str!("part1.data")
        .lines()
        .map(|l| l.into())
        .collect();

    input.sort_by(|a, b| RANKS_P1[&a.to_string()].cmp(&RANKS_P1[&b.to_string()]));

    let total: usize = input
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            println!("{}: {} with bid {}", i + 1, hand, hand.bid);
            hand.bid * (i + 1)
        })
        .sum();

    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {}
