use std::collections::HashMap;

pub fn main() {
    let input = include_str!("part1.data").trim().split(',');

    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    input.for_each(|s| {
        if let Some((label, lens_idx)) = s.split_once('=') {
            let value = lens_idx.parse::<usize>().unwrap();
            let hash = hash(label);
            boxes
                .entry(hash)
                .and_modify(|b| match b.iter_mut().find(|lens| lens.0 == label) {
                    Some(lens) => lens.1 = value,
                    None => b.push((label.to_string(), value)),
                })
                .or_insert(vec![(label.to_string(), value)]);
        } else if let Some(label) = s.strip_suffix('-') {
            let hash = hash(label);
            boxes
                .entry(hash)
                .and_modify(|b| b.retain(|lens| lens.0 != label));
        } else {
            panic!("invalid input");
        }
    });

    let focusing_power = boxes
        .iter()
        .map(|(k, b)| {
            b.iter()
                .enumerate()
                .map(|(idx, (_, focal_length))| (k + 1) * (idx + 1) * focal_length)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("result : {}", focusing_power);
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0usize, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

#[cfg(test)]
mod tests {}
