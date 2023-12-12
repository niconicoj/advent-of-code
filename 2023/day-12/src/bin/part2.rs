use itertools::Itertools;

pub fn main() {
    let input = include_str!("part1.data").lines();

    let result = input
        .map(|l| {
            let (map, groups) = l.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .filter_map(|g| g.parse::<usize>().ok())
                .collect::<Vec<_>>();

            let possibilities = all_possibilities(&groups, map.len()).unwrap();

            possibilities
                .iter()
                .filter(|p| is_possible_for(p, map))
                .count()
        })
        .sum::<usize>();

    println!("result: {}", result);
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

fn all_possibilities(groups: &[usize], space: usize) -> Result<Vec<String>, String> {
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
