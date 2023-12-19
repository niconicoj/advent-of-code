use std::collections::HashSet;

pub fn main() {
    let input = include_str!("part1.data").lines();

    let mut map: HashSet<(isize, isize)> = HashSet::new();

    let mut current_position = (0, 0);

    input.for_each(|line| {
        let (direction, rest) = line.split_once(' ').expect("bad format");
        let (distance, _) = rest.split_once(' ').expect("bad format");

        let distance = distance.parse::<isize>().expect("bad format");

        match direction {
            "U" => {
                (current_position.1..=(current_position.1 + distance)).for_each(|y| {
                    map.insert((current_position.0, y));
                });
                current_position = (current_position.0, current_position.1 + distance);
            }
            "D" => {
                (current_position.1 - distance..=current_position.1).for_each(|y| {
                    map.insert((current_position.0, y));
                });
                current_position = (current_position.0, current_position.1 - distance);
            }
            "L" => {
                (current_position.0 - distance..=current_position.0).for_each(|x| {
                    map.insert((x, current_position.1));
                });
                current_position = (current_position.0 - distance, current_position.1);
            }
            "R" => {
                (current_position.0..=current_position.0 + distance).for_each(|x| {
                    map.insert((x, current_position.1));
                });
                current_position = (current_position.0 + distance, current_position.1);
            }
            _ => panic!("bad format"),
        };
    });

    let x_range = (
        map.iter().map(|v| v.0).min().unwrap(),
        map.iter().map(|v| v.0).max().unwrap(),
    );

    let y_range = (
        map.iter().map(|v| v.1).min().unwrap(),
        map.iter().map(|v| v.1).max().unwrap(),
    );

    println!("x range : {:?}", x_range);
    println!("y range : {:?}", y_range);

    println!("map :");
    println!("{:?}", map);

    (y_range.0..=y_range.1).rev().for_each(|y| {
        println!();
        (x_range.0..=x_range.1).for_each(|x| {
            print!("{}", if map.contains(&(x, y)) { '#' } else { '.' });
        })
    });

    println!();
    print!("computed area : ");
    let total_area = compute_area(&x_range, &y_range, &map);

    println!();
    println!("total area : {}", total_area);
}

fn compute_area(
    x_range: &(isize, isize),
    y_range: &(isize, isize),
    map: &HashSet<(isize, isize)>,
) -> usize {
    (y_range.0..=y_range.1)
        .rev()
        .map(|y| {
            let mut ray_state = RayState::default();
            println!();
            (x_range.0..=x_range.1)
                .filter(|&x| {
                    let is_inside = is_inside(&(x, y), &mut ray_state, &map);
                    print!("{}", if is_inside { '#' } else { '.' });
                    is_inside
                })
                .count()
        })
        .sum()
}

#[derive(Default)]
struct RayState(bool, Edge);

#[derive(Default)]
enum Edge {
    #[default]
    None,
    Rising,
    Falling,
}

fn is_inside(
    position: &(isize, isize),
    state: &mut RayState,
    map: &HashSet<(isize, isize)>,
) -> bool {
    let window = (
        map.contains(&(position.0, position.1 + 1)),
        map.contains(&(position.0, position.1)),
        map.contains(&(position.0, position.1 - 1)),
    );
    match (window, &state) {
        ((false, true, false), _) => true,
        ((_, false, _), _) => state.0,
        ((true, true, false), RayState(_, Edge::None)) => {
            state.1 = Edge::Falling;
            true
        }
        ((true, true, false), RayState(_, Edge::Falling)) => {
            state.1 = Edge::None;
            true
        }
        ((true, true, false), RayState(_, Edge::Rising)) => {
            state.0 = !state.0;
            state.1 = Edge::None;
            true
        }
        ((false, true, true), RayState(_, Edge::None)) => {
            state.1 = Edge::Rising;
            true
        }
        ((false, true, true), RayState(_, Edge::Falling)) => {
            state.0 = !state.0;
            state.1 = Edge::None;
            true
        }
        ((false, true, true), RayState(_, Edge::Rising)) => {
            state.1 = Edge::None;
            true
        }
        ((true, true, true), _) => {
            state.0 = !state.0;
            true
        }
    }
}

#[cfg(test)]
mod tests {}
