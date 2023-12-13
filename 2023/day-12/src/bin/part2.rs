#![feature(iter_intersperse)]

use itertools::Itertools;

pub fn main() {
    let input = include_str!("part1-test.data").lines();

    let result = input
        .map(|l| {
            let (map, groups) = process(l);

            let possibilities = all_possibilities(&map, &groups).unwrap();

            possibilities
                .iter()
                .filter(|p| is_possible_for(p, &map))
                .count()
        })
        .sum::<usize>();

    println!("result: {}", result);
}

fn process(line: &str) -> (String, Vec<usize>) {
    let (map, groups) = line.split_once(' ').unwrap();
    let groups = groups
        .split(',')
        .filter_map(|g| g.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let map = (0..5).map(|_| map.to_owned()).join("?");
    let groups = (0..5).flat_map(|_| groups.clone()).collect();

    (map, groups)
}

fn is_possible_for(possibility: &str, map: &str) -> bool {
    if possibility.len() != map.len() {
        false
    } else {
        possibility
            .chars()
            .zip(map.chars())
            .all(|(p, m)| m == '?' || p == m)
    }
}

fn all_possibilities(map: &str, groups: &[usize]) -> Result<Vec<String>, String> {
    let (known_parts, unkown_parts) = slice_into_parts(map);

    println!("map : {}", map);
    println!("known parts: {:?}", known_parts);
    println!("unkown parts parts: {:?}", unkown_parts);
    Ok(vec![])
}

fn slice_into_parts(map: &str) -> (Vec<Option<&str>>, Vec<usize>) {
    let known_parts = map
        .split('?')
        .map(|l| if l.is_empty() { None } else { Some(l) });
    let mut known_parts = Iterator::intersperse_with(known_parts, || None).collect::<Vec<_>>();
    known_parts.dedup();

    let unkown_parts = map
        .split(|c| c != '?')
        .map(|l| l.len())
        .filter(|&l| l != 0)
        .collect::<Vec<_>>();

    (known_parts, unkown_parts)
}

fn _all_possibilities(groups: &[usize], space: usize) -> Result<Vec<String>, String> {
    check_inputs(groups, space)?;
    let empty_space = space + 1 - groups.iter().sum::<usize>() - groups.len();

    let possibilities = (0..(groups.len() + empty_space))
        .combinations(groups.len())
        .map(|possibility| {
            let mut s: Vec<char> = vec!['.'; space];
            let mut ptr = 0;
            for (&i, &j) in possibility.iter().zip(groups.iter()) {
                let start = i + ptr;
                let end = i + j + ptr;
                (start..end).for_each(|k| s[k] = '#');
                ptr += j;
            }
            s.iter().collect::<String>()
        })
        .collect::<Vec<_>>();

    Ok(possibilities)
}

fn check_inputs(groups: &[usize], space: usize) -> Result<(), String> {
    if space == 0 {
        return Err("Space must be greater than 0".into());
    }
    let min_space = (groups.iter().sum::<usize>() + groups.len())
        .checked_sub(1)
        .unwrap_or_default();
    if space < min_space {
        return Err(format!(
            "Space must be greater than {} (min_space)",
            min_space
        ));
    }
    if groups.is_empty() {
        return Err(format!("Groups must not be empty"));
    }
    Ok(())
}
