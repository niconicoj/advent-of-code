use std::fmt::Display;

pub fn main() {
    let input: &str = include_str!("part1.data");

    let height = input.lines().count();
    let width = input.chars().count() / height;

    let result = solve(input, height, width);

    println!("{}", result);
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
    println!("height: {}, width: {}", height, width);
    let start = input
        .chars()
        .position(|c| c == 'S')
        .expect("No start point found");

    println!("start: {:?}", start);

    let mut current = start;
    let mut direction = Direction::Up;

    find_first_pipe(input, width, &mut current, &mut direction);
    let mut traversed = 1;

    while current != start {
        travel(input, width, &mut current, &mut direction);
        traversed += 1;
    }

    (traversed / 2).to_string()
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
        (Direction::Up, *current - width),
        (Direction::Down, *current + width),
        (Direction::Left, *current - 1),
        (Direction::Right, *current + 1),
    ] {
        if let Some(next_pipe) = input.chars().nth(p) {
            if d.is_valid_for_pipe(next_pipe) {
                // println!("Next pipe: {}", next_pipe);
                *direction = d.next(&next_pipe);
                // println!("Direction : {}", direction);
                // println!("────────────────────────");
                *current = p;
            }
        }
    }
}

#[cfg(test)]
mod tests {}
