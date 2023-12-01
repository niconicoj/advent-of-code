pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

fn solve(input: Vec<&str>) -> String {
    input
        .iter()
        .map(|l| {
            let s = format!("{}{}",
                l.chars().find(|c| c.is_numeric()).unwrap(),
                l.chars().rev().find(|c| c.is_numeric()).unwrap(),
            );
            s.parse::<usize>().unwrap()
        })
            .reduce(|acc, v| acc + v)
        .unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn day1_part1() {
        let input: Vec<_> = include_str!("part1-test.data").lines().collect();
        assert_eq!(solve(input), "142");
    }

}
