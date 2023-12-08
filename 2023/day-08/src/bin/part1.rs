use std::collections::HashMap;

const TRIM_PATTERN: &[char] = &['(', ')'];

pub fn main() {
    let input = include_str!("part1.data").lines();

    let result = solve(input.collect());

    println!("result: {}", result);
}

fn solve(input: Vec<&str>) -> String {
    let mut iter = input.iter();

    let mut direction = iter.next().expect("direction not found").chars().cycle();

    // discard empty line
    iter.next().unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    iter.for_each(|line| {
        let (origin, el) = line.split_once(" = ").expect("invalid line");
        let (left, right) = el
            .trim_matches(TRIM_PATTERN)
            .split_once(", ")
            .expect("invalid line");

        map.insert(origin, (left, right));
    });

    let mut current: &str = "AAA";
    let mut count = 0;

    while current != "ZZZ" {
        let (left, right) = map.get(current).expect("invalid line");
        if direction.next().unwrap() == 'L' {
            current = left;
        } else {
            current = right;
        }
        count += 1;
    }

    count.to_string()
}

#[cfg(test)]
mod tests {}
