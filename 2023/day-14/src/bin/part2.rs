use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

pub fn main() {
    let input = include_str!("part1.data").lines();

    let mut columns: Vec<Vec<char>> = vec![];

    input.for_each(|line| {
        line.chars()
            .enumerate()
            .for_each(|(i, c)| match columns.get_mut(i) {
                Some(column) => column.push(c),
                None => columns.push(vec![c]),
            })
    });

    let mut dish = Dish::new(columns);

    let mut cache: HashMap<Dish, usize> = HashMap::new();
    let mut cycles = 0;

    while cache.get(&dish).is_none() {
        cycles += 1;
        cache.insert(dish.clone(), cycles);
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .iter()
        .for_each(|&dir| {
            dish.tilt(dir);
        });
    }

    println!("dish repeated after {} cycles", cycles);
    println!(
        "last iteration was after {} cycles",
        cache.get(&dish).unwrap()
    );

    let offset = cache.get(&dish).unwrap() - 1;
    let frequency = cycles - cache.get(&dish).unwrap() + 1;

    println!("frequency: {}", frequency);
    println!("offset: {}", offset);

    let cycles_left = (1_000_000_000 - offset + frequency) % frequency;

    println!("cycles left: {}", cycles_left);

    (0..cycles_left).for_each(|_| {
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .iter()
        .for_each(|&dir| {
            dish.tilt(dir);
        });
    });

    println!("result: {}", dish.measure());
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
struct Dish {
    columns: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Dish {
    fn new(columns: Vec<Vec<char>>) -> Self {
        Self { columns }
    }

    fn measure(&self) -> usize {
        self.columns
            .iter()
            .map(|column| Self::compute_column(column.to_vec()))
            .sum::<usize>()
    }

    fn compute_column(column: Vec<char>) -> usize {
        column
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, c)| match c {
                'O' => Some(i + 1),
                _ => None,
            })
            .sum::<usize>()
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.tilt_north(),
            Direction::South => self.tilt_south(),
            Direction::East => self.tilt_east(),
            Direction::West => self.tilt_west(),
        }
    }

    fn tilt_north(&mut self) {
        self.columns.iter_mut().for_each(|column| {
            Self::tilt_column_north(column);
        });
    }

    fn tilt_south(&mut self) {
        self.columns.iter_mut().for_each(|column| {
            Self::tilt_column_south(column);
        });
    }

    fn tilt_west(&mut self) {
        (0..self.columns.len()).for_each(|x| {
            let mut current_offset = 0;
            let new_indexes = self
                .columns
                .iter_mut()
                .enumerate()
                .filter_map(|(i, c)| match c.get_mut(x) {
                    Some(c) if c == &'O' => {
                        let rock_offset = current_offset;
                        current_offset += 1;
                        *c = '.';
                        Some(rock_offset)
                    }
                    Some('#') => {
                        current_offset = i + 1;
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            new_indexes.iter().for_each(|offset| {
                if let Some(location) = self.columns.get_mut(*offset).and_then(|c| c.get_mut(x)) {
                    *location = 'O';
                }
            });
        });
    }

    fn tilt_east(&mut self) {
        (0..self.columns.len()).for_each(|x| {
            let mut current_offset = self.columns.len() - 1;
            let new_indexes = self
                .columns
                .iter_mut()
                .enumerate()
                .rev()
                .filter_map(|(i, c)| match c.get_mut(x) {
                    Some(c) if c == &'O' => {
                        let rock_offset = current_offset;
                        current_offset = current_offset.checked_sub(1).unwrap_or(0);
                        *c = '.';
                        Some(rock_offset)
                    }
                    Some('#') => {
                        current_offset = i.checked_sub(1).unwrap_or(0);
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            new_indexes.iter().for_each(|offset| {
                if let Some(location) = self.columns.get_mut(*offset).and_then(|c| c.get_mut(x)) {
                    *location = 'O';
                }
            });
        });
    }

    fn tilt_column_north(column: &mut Vec<char>) {
        let mut current_offset = 0;
        let new_indexes = column
            .iter_mut()
            .enumerate()
            .filter_map(|(i, c)| match c {
                'O' => {
                    let rock_offset = current_offset;
                    current_offset += 1;
                    *c = '.';
                    Some(rock_offset)
                }
                '#' => {
                    current_offset = i + 1;
                    None
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        new_indexes.iter().for_each(|offset| {
            if let Some(location) = column.get_mut(*offset) {
                *location = 'O';
            }
        });
    }

    fn tilt_column_south(column: &mut Vec<char>) {
        let mut current_offset = column.len() - 1;
        let new_indexes = column
            .iter_mut()
            .enumerate()
            .rev()
            .filter_map(|(i, c)| match c {
                'O' => {
                    let rock_offset = current_offset;
                    current_offset = current_offset.checked_sub(1).unwrap_or(0);
                    *c = '.';
                    Some(rock_offset)
                }
                '#' => {
                    current_offset = i.checked_sub(1).unwrap_or(0);
                    None
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        new_indexes.iter().for_each(|offset| {
            if let Some(location) = column.get_mut(*offset) {
                *location = 'O';
            }
        });
    }
}

impl Display for Dish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.columns.len()).try_for_each(|x| {
            self.columns.iter().try_for_each(|c| {
                write!(f, "{}", c[x])?;
                Ok(())
            })?;
            writeln!(f, " {}", self.columns.len() - x)
        })
    }
}

#[cfg(test)]
mod tests {}
