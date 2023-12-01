#[rustfmt::skip]
static PATTERNS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", 
    "one", "two", "three", "four", "five", "six","seven", "eight", "nine",
];

pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

fn solve(input: Vec<&str>) -> String {
    let res: usize = input.iter().map(|l| {
        let first = PATTERNS
            .iter()
            .map(|p| (p, l.find(p)))
            .filter(|(_, m)| m.is_some())
            .min_by(|(_, a), (_, b)| a.unwrap().cmp(&b.unwrap()))
            .map(|(s, _)| parse_number(s))
            .expect("could not find a match number");

        let last = PATTERNS
            .iter()
            .map(|p| (p, l.rfind(p)))
            .filter(|(_, m)| m.is_some())
            .max_by(|(_, a), (_, b)| a.unwrap().cmp(&b.unwrap()))
            .map(|(s, _)| parse_number(s))
            .expect("could not find a match number");

        let v = format!("{}{}", first, last);
        println!("{v}");
        v.parse::<usize>().unwrap()
    }).sum();
    res.to_string()
}

fn parse_number(s: &str) -> usize {
    match s {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("no matching number")
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn day1_part2() {
        let input: Vec<_> = include_str!("part2-test.data").lines().collect();
        assert_eq!(solve(input), "281");
    }

    #[test]
    fn day1_part2_edge1() {
        assert_eq!(solve(vec!["eighthree"]), "83");
    }

    #[test]
    fn day1_part2_edge2() {
        assert_eq!(solve(vec!["sevenine"]), "79");
    }
}
