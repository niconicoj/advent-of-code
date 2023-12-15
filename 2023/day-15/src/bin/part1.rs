pub fn main() {
    let input = include_str!("part1.data").trim().split(',');

    let result: usize = input
        .map(|s| {
            s.chars()
                .fold(0usize, |acc, c| ((acc + (c as usize)) * 17) % 256)
        })
        .sum();

    println!("result: {}", result);
}

#[cfg(test)]
mod tests {}
