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
    let mut origins = Vec::new();

    iter.for_each(|line| {
        let (origin, el) = line.split_once(" = ").expect("invalid line");

        if &origin[2..3] == "A" {
            origins.push(origin);
        }

        let (left, right) = el
            .trim_matches(TRIM_PATTERN)
            .split_once(", ")
            .expect("invalid line");

        map.insert(origin, (left, right));
    });

    let results = origins
        .iter()
        .map(|&origin| {
            let mut origin = origin;
            let mut count = 0;

            while &origin[2..3] != "Z" {
                let (left, right) = map.get(origin).expect("invalid line");
                if direction.next().unwrap() == 'L' {
                    origin = left;
                } else {
                    origin = right;
                }
                count += 1;
            }

            count
        })
        .collect::<Vec<_>>();

    println!("results: {:?}", results);

    let result = lcm(results);

    result.to_string()
}

fn lcm(mut numbers: Vec<u64>) -> u64 {
    let mut result = numbers.pop().unwrap();
    for number in numbers {
        result = result * number / gcd(result, number);
    }
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {}
