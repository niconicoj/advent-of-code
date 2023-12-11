use std::collections::BTreeSet;

pub fn main() {
    let input: &str = include_str!("part1.data");

    let universe = process(input);
    let result = run(universe);

    println!("result: {}", result);
}

fn run(universe: Vec<(isize, isize)>) -> isize {
    let mut sum = 0;
    for i in 0..universe.len() {
        for j in (i + 1)..universe.len() {
            let (x1, y1) = universe[i];
            let (x2, y2) = universe[j];

            let distance = (x1 - x2).abs() + (y1 - y2).abs();
            sum += distance;
        }
    }
    sum
}

pub fn process(input: &str) -> Vec<(isize, isize)> {
    let mut occupied_x: BTreeSet<isize> = BTreeSet::new();
    let mut occupied_y: BTreeSet<isize> = BTreeSet::new();
    let mut universe: Vec<(isize, isize)> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| {
                occupied_x.insert(x as isize);
                occupied_y.insert(y.clone() as isize);
                universe.push((x as isize, y as isize));
            });
    });

    // update the universe by expanding unoccupied row and column by twice their size
    expand_x(occupied_x, &mut universe);
    expand_y(occupied_y, &mut universe);

    universe
}

fn expand_x(occupied_x: BTreeSet<isize>, universe: &mut Vec<(isize, isize)>) {
    match (occupied_x.first(), occupied_x.last()) {
        (Some(&min), Some(&max)) => (min..max).rev().for_each(|col| {
            if !occupied_x.contains(&col) {
                universe.iter_mut().for_each(|(x, _)| {
                    if *x > col {
                        *x += 999999;
                    }
                });
            }
        }),
        (None, None) => println!("nothing to expand"),
        _ => unreachable!(""),
    }
}

fn expand_y(occupied_y: BTreeSet<isize>, universe: &mut Vec<(isize, isize)>) {
    match (occupied_y.first(), occupied_y.last()) {
        (Some(&min), Some(&max)) => (min..max).rev().for_each(|row| {
            if !occupied_y.contains(&row) {
                universe.iter_mut().for_each(|(_, y)| {
                    if *y > row {
                        *y += 999999;
                    }
                });
            }
        }),
        (None, None) => println!("nothing to expand"),
        _ => unreachable!(""),
    }
}

#[cfg(test)]
mod tests {}
