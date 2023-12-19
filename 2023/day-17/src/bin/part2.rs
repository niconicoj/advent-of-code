use std::collections::HashMap;

use rayon::prelude::*;

pub fn main() {
    let input = include_str!("part1.data").lines();
    let map: Vec<Vec<char>> = input.map(|s| s.chars().collect()).collect();

    let height = map.len();
    let width = map[0].len();

    let result = [
        Direction::Right,
        Direction::Left,
        Direction::Down,
        Direction::Up,
    ]
    .par_iter()
    .flat_map(|&direction| {
        let range = match direction {
            Direction::Right | Direction::Left => 0..width,
            Direction::Down | Direction::Up => 0..height,
        };
        range
            .map(|v| {
                let mut energized: HashMap<(usize, usize), Direction> = HashMap::new();

                let initial_beam = match direction {
                    Direction::Right => Beam::new((0, v), direction),
                    Direction::Left => Beam::new((width - 1, v), direction),
                    Direction::Down => Beam::new((v, 0), direction),
                    Direction::Up => Beam::new((v, height - 1), direction),
                };
                energized.insert(initial_beam.position, initial_beam.direction);
                compute((Some(initial_beam), None), &map, &mut energized);
                energized.len()
            })
            .collect::<Vec<_>>()
    })
    .max();

    println!("result : {:?}", result);
}

fn compute(
    beams: (Option<Beam>, Option<Beam>),
    map: &Vec<Vec<char>>,
    energized: &mut HashMap<(usize, usize), Direction>,
) {
    match beams {
        (None, None) => return,
        (Some(new_beam), None) => {
            compute(new_beam.advance(map, energized), map, energized);
        }
        (Some(new_beam1), Some(new_beam2)) => {
            compute(new_beam1.advance(map, energized), map, energized);
            compute(new_beam2.advance(map, energized), map, energized);
        }
        _ => unreachable!("Single beam are always Some(_), None"),
    }
}

#[derive(Debug)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}

impl Beam {
    fn new(position: (usize, usize), direction: Direction) -> Self {
        Beam {
            position,
            direction,
        }
    }
    fn default() -> Self {
        Beam {
            position: (0, 0),
            direction: Direction::Right,
        }
    }

    fn get_cell<'a>(&'a self, map: &'a Vec<Vec<char>>) -> Option<&'a char> {
        map.get(self.position.1)
            .and_then(|row| row.get(self.position.0))
    }

    fn advance(
        mut self,
        map: &Vec<Vec<char>>,
        energized: &mut HashMap<(usize, usize), Direction>,
    ) -> (Option<Beam>, Option<Beam>) {
        // handle mirrors
        self.direction = match (self.get_cell(map), self.direction) {
            (Some(&'/'), Direction::Up) => Direction::Right,
            (Some(&'/'), Direction::Down) => Direction::Left,
            (Some(&'/'), Direction::Left) => Direction::Down,
            (Some(&'/'), Direction::Right) => Direction::Up,
            (Some(&'\\'), Direction::Up) => Direction::Left,
            (Some(&'\\'), Direction::Down) => Direction::Right,
            (Some(&'\\'), Direction::Left) => Direction::Up,
            (Some(&'\\'), Direction::Right) => Direction::Down,
            _ => self.direction,
        };

        match self.direction {
            Direction::Up => {
                self.position.1 = match self.position.1.checked_sub(1) {
                    Some(y) => y,
                    None => return (None, None),
                }
            }
            Direction::Down => self.position.1 += 1,
            Direction::Left => {
                self.position.0 = match self.position.0.checked_sub(1) {
                    Some(x) => x,
                    None => return (None, None),
                }
            }
            Direction::Right => self.position.0 += 1,
        };
        let new_cell = self.get_cell(map);

        if new_cell.is_none() {
            return (None, None);
        }

        if energized
            .get(&self.position)
            .map(|d| *d == self.direction)
            .unwrap_or(false)
        {
            return (None, None);
        } else {
            energized.insert(self.position, self.direction);
        }

        match (new_cell, self.direction) {
            (Some(&'|'), Direction::Left) | (Some(&'|'), Direction::Right) => (
                Some(Self {
                    position: self.position,
                    direction: Direction::Up,
                }),
                Some(Self {
                    position: self.position,
                    direction: Direction::Down,
                }),
            ),
            (Some(&'-'), Direction::Up) | (Some(&'-'), Direction::Down) => (
                Some(Self {
                    position: self.position,
                    direction: Direction::Left,
                }),
                Some(Self {
                    position: self.position,
                    direction: Direction::Right,
                }),
            ),
            _ => (Some(self), None),
        }

        // check if we already energized this spot in the same direction
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {}
