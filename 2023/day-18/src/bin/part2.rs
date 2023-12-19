pub fn main() {
    let input = include_str!("part1.data")
        .lines()
        .take_while(|l| !l.is_empty());

    let mut edges: Vec<(isize, Direction)> = Vec::new();

    let patterns: &[_] = &['(', '#', ')'];

    input.for_each(|line| {
        let instruction = line
            .split_once(' ')
            .and_then(|(_, l)| l.split_once(' '))
            .unwrap()
            .1
            .trim_matches(patterns);

        let (distance, direction) = instruction.split_at(instruction.len() - 1);

        let direction = Direction::from(direction);
        let distance = isize::from_str_radix(distance, 16).unwrap();

        match direction {
            Direction::Up => {
                edges.push((distance, Direction::Up));
            }
            Direction::Down => {
                edges.push((-distance, Direction::Down));
            }
            Direction::Left => {
                edges.push((-distance, Direction::Left));
            }
            Direction::Right => {
                edges.push((distance, Direction::Right));
            }
        };
    });

    arrange(&mut edges);

    let points = edges_to_points(edges);

    println!("{}", compute_area(points));
}

fn compute_area(points: Vec<(isize, isize)>) -> isize {
    points
        .windows(2)
        .map(|pts| {
            if let [p1, p2] = pts {
                (p1.0 + p2.0) * (p1.1 - p2.1)
            } else {
                panic!("bad format")
            }
        })
        .sum::<isize>()
        / 2
}

fn edges_to_points(edges: Vec<(isize, Direction)>) -> Vec<(isize, isize)> {
    let mut points = Vec::new();
    let mut current = (0, 0);
    points.push(current);

    edges
        .iter()
        .for_each(|(distance, direction)| match direction {
            Direction::Up | Direction::Down => {
                current.1 += *distance;
                points.push(current);
            }
            Direction::Left | Direction::Right => {
                current.0 += *distance;
                points.push(current);
            }
        });

    points
}

fn arrange(edges: &mut Vec<(isize, Direction)>) {
    for idx in 0..edges.len() {
        let next = (idx + 1) % edges.len();
        let prev = (idx + edges.len() - 1) % edges.len();

        let next_direction = edges.get(next).unwrap().1;
        let prev_direction = edges.get(prev).unwrap().1;

        match edges.get_mut(idx).unwrap() {
            (d, Direction::Up) => match (prev_direction, next_direction) {
                (Direction::Left, Direction::Right) => {
                    *d += 1;
                }
                (Direction::Right, Direction::Left) => {
                    *d -= 1;
                }
                _ => {}
            },
            (d, Direction::Down) => match (prev_direction, next_direction) {
                (Direction::Left, Direction::Right) => {
                    *d += 1;
                }
                (Direction::Right, Direction::Left) => {
                    *d -= 1;
                }
                _ => {}
            },
            (d, Direction::Left) => match (prev_direction, next_direction) {
                (Direction::Up, Direction::Down) => {
                    *d += 1;
                }
                (Direction::Down, Direction::Up) => {
                    *d -= 1;
                }
                _ => {}
            },

            (d, Direction::Right) => match (prev_direction, next_direction) {
                (Direction::Up, Direction::Down) => {
                    *d += 1;
                }
                (Direction::Down, Direction::Up) => {
                    *d -= 1;
                }
                _ => {}
            },
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "3" => Self::Up,
            "1" => Self::Down,
            "2" => Self::Left,
            "0" => Self::Right,
            _ => panic!("bad format"),
        }
    }
}

#[cfg(test)]
mod tests {}
