use itertools::Itertools;
use rayon::prelude::*;

pub fn main() {
    let input = include_str!("part1.data").par_lines();

    let result: usize = input
        .map(|l| {
            let (map, groups) = process(l);

            possibilities(&map, &groups)
        })
        .sum();

    println!("result: {}", result);
}

fn process(line: &str) -> (String, Vec<usize>) {
    let (map, groups) = line.split_once(' ').unwrap();
    let groups = groups
        .split(',')
        .filter_map(|g| g.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let map = (0..5).map(|_| map.to_owned()).join("?");
    let groups = (0..5).flat_map(|_| groups.clone()).collect::<Vec<_>>();

    (map, groups)
}

fn possibilities(map: &str, groups: &[usize]) -> usize {
    let max_group = groups.iter().max().unwrap();

    let mut dp: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![0; *max_group + 1]; groups.len() + 1]; map.len() + 1];

    for i in 0..=map.len() {
        let x = map.chars().nth(i).unwrap_or('.');
        for j in 0..=groups.len() {
            let group = groups.get(j).unwrap_or(&0);
            for k in 0..=*group {
                if i == 0 {
                    let value = match (j, k, x) {
                        (j, _, _) if j != 0 => 0,
                        (_, k, '#') if k == 1 => 1,
                        (_, k, '.') if k == 0 => 1,
                        (_, k, '?') if [0, 1].contains(&k) => 1,
                        _ => 0,
                    };
                    dp[i][j][k] = value;
                    continue;
                }

                let dot_possibilities = match (i, j, k) {
                    (_, _, k) if k != 0 => 0,
                    (_, j, _) if j > 0 => dp[i - 1][j - 1][groups[j - 1]] + dp[i - 1][j][0],
                    _ => {
                        if !map[..i].contains('#') {
                            1
                        } else {
                            0
                        }
                    }
                };

                let pound_possibilites = if k == 0 { 0 } else { dp[i - 1][j][k - 1] };

                match x {
                    '.' => dp[i][j][k] = dot_possibilities,
                    '#' => dp[i][j][k] = pound_possibilites,
                    '?' => dp[i][j][k] = dot_possibilities + pound_possibilites,
                    _ => panic!("unexpected character"),
                }
            }
        }
    }

    dp[map.len()][groups.len()][0]
}
