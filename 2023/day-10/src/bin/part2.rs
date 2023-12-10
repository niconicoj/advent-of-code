use std::{collections::HashSet, fmt::Display};

pub fn main() {
    let input: &str = include_str!("part1.data");

    let height = input.lines().count();
    let width = input.chars().count() / height;

    let result = solve(input, height, width);

    println!("result = {}", result);
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_valid_for_pipe(&self, pipe: char) -> bool {
        let pattern: &[char] = match self {
            Direction::Up => &['|', '┌', '┐'],
            Direction::Down => &['|', '└', '┘'],
            Direction::Left => &['─', '┌', '└'],
            Direction::Right => &['─', '┐', '┘'],
        };

        pattern.contains(&pipe)
    }

    fn next(&self, next_pipe: &char) -> Self {
        match (self, &next_pipe) {
            (Direction::Up, '│') => Direction::Up,
            (Direction::Up, '┌') => Direction::Right,
            (Direction::Up, '┐') => Direction::Left,
            (Direction::Down, '│') => Direction::Down,
            (Direction::Down, '└') => Direction::Right,
            (Direction::Down, '┘') => Direction::Left,
            (Direction::Left, '─') => Direction::Left,
            (Direction::Left, '┌') => Direction::Down,
            (Direction::Left, '└') => Direction::Up,
            (Direction::Right, '─') => Direction::Right,
            (Direction::Right, '┐') => Direction::Down,
            (Direction::Right, '┘') => Direction::Up,
            _ => unreachable!("Unexpected pipe {} for direction {}", next_pipe, self),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => '↑',
            Direction::Down => '↓',
            Direction::Left => '←',
            Direction::Right => '→',
        };
        write!(f, "{}", c)
    }
}

fn solve(input: &str, height: usize, width: usize) -> String {
    let start = input
        .chars()
        .position(|c| c == 'S')
        .expect("No start point found");

    let mut current = start;
    let mut direction = Direction::Up;

    let mut set: HashSet<usize> = HashSet::new();

    println!("Computing maze...");
    set.insert(current);
    find_first_pipe(input, width, &mut current, &mut direction);

    while current != start {
        set.insert(current);
        travel(input, width, &mut current, &mut direction);
    }

    println!("finding inside points...");

    let count = compute_inside_points(input, width, &set);

    count.to_string()
}

enum Edge {
    No,
    Down,
    Up,
}

enum State {
    Inside,
    Outside,
}

impl State {
    fn invert(&self) -> Self {
        match self {
            State::Inside => State::Outside,
            State::Outside => State::Inside,
        }
    }
}

fn compute_inside_points(input: &str, width: usize, set: &HashSet<usize>) -> usize {
    input
        .chars()
        .enumerate()
        .filter(|&(i, _)| {
            if let Some(_) = set.get(&i) {
                return false;
            }
            let mut state = State::Outside;
            let mut edge = Edge::No;
            let mut intersection = 0;
            ((i - (i % width))..i).into_iter().for_each(|i| {
                if let Some(_) = set.get(&i) {
                    let c = input
                        .chars()
                        .nth(i)
                        .map(|c| if c == 'S' { '─' } else { c })
                        .expect("no char found");

                    match (&state, &edge, c) {
                        (_, Edge::No, '┌') => {
                            edge = Edge::Up;
                        }
                        (_, Edge::Up, '┐') => {
                            edge = Edge::No;
                        }
                        (_, Edge::Up, '┘') => {
                            state = state.invert();
                            intersection += 1;
                            edge = Edge::No;
                        }
                        (_, Edge::No, '└') => {
                            edge = Edge::Down;
                        }
                        (_, Edge::Down, '┘') => {
                            edge = Edge::No;
                        }
                        (_, Edge::Down, '┐') => {
                            state = state.invert();
                            intersection += 1;
                            edge = Edge::No;
                        }
                        (_, _, '│') => {
                            state = state.invert();
                            intersection += 1;
                        }
                        (_, _, '─') => {}
                        _ => panic!("Unexpected char {} at ({},{})", c, i % width, i / width),
                    }
                }
            });
            let inside = intersection % 2 == 1;
            if inside {
                println!("point ({},{}) is inside", i % width, i / width);
            }
            inside
        })
        .count()
}

fn travel(input: &str, width: usize, current: &mut usize, direction: &mut Direction) {
    let next = match direction {
        Direction::Up => *current - width,
        Direction::Down => *current + width,
        Direction::Left => *current - 1,
        Direction::Right => *current + 1,
    };

    let next_pipe = input.chars().nth(next).expect("No next pipe found");
    if next_pipe == 'S' {
        *current = next;
        return;
    }

    // println!("Next pipe : {}", next_pipe);
    *direction = direction.next(&next_pipe);
    // println!("Direction : {}", direction);
    // println!("────────────────────────");
    *current = next;
}

fn find_first_pipe(input: &str, width: usize, current: &mut usize, direction: &mut Direction) {
    for (d, p) in [
        (Direction::Up, current.checked_sub(width)),
        (Direction::Down, current.checked_add(width)),
        (Direction::Left, current.checked_sub(1)),
        (Direction::Right, current.checked_add(1)),
    ] {
        if p.is_none() {
            continue;
        }
        if let Some(next_pipe) = input.chars().nth(p.unwrap()) {
            if d.is_valid_for_pipe(next_pipe) {
                // println!("Next pipe: {}", next_pipe);
                *direction = d.next(&next_pipe);
                // println!("Direction : {}", direction);
                // println!("────────────────────────");
                *current = p.unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {}
