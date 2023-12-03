use std::collections::HashMap;

pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

type Coord = (isize, isize);

#[derive(Debug)]
struct Number {
    id: isize,
    value: isize,
}

#[derive(Debug)]
pub struct Schematic {
    gears: Vec<Coord>,
    numbers: HashMap<Coord, Number>,
}

impl Schematic {
    pub fn new() -> Self {
        Self {
            gears: Vec::new(),
            numbers: HashMap::new(),
        }
    }

    pub fn parse(input: Vec<&str>) -> Schematic {
        let mut schematic = Self::new();
        let mut number_id = 0;
        for (y, line) in input.iter().enumerate() {
            let mut line_iter = line.chars().enumerate().peekable();
            while let Some((x, c)) = line_iter.next() {
                match c {
                    '*' => schematic.gears.push((x as isize, y as isize)),
                    c if c.is_numeric() => {
                        let mut number = String::from(c);
                        while let Some((_, c)) = line_iter.peek() {
                            if c.is_numeric() {
                                number.push(line_iter.next().unwrap().1);
                            } else {
                                break;
                            }
                        }
                        (x..(x + number.len())).for_each(|x| {
                            schematic.numbers.insert(
                                (x as isize, y as isize),
                                Number {
                                    id: number_id,
                                    value: number.parse().unwrap(),
                                },
                            );
                        });
                        number_id += 1;
                    }
                    _ => continue,
                }
            }
        }
        schematic
    }

    pub fn compute_gear_ratios(&self) -> Vec<isize> {
        self.gears
            .iter()
            .map(|(x, y)| {
                let mut numbers = HashMap::new();
                (-1..=1).for_each(|x_offset: isize| {
                    (-1..=1).for_each(|y_offset: isize| {
                        let coord = (x + x_offset, y + y_offset);
                        if let Some(number) = self.numbers.get(&coord) {
                            println!(
                                "number with id {} and value {} is found adjacent to gear {:?}",
                                number.id,
                                number.value,
                                (x, y)
                            );
                            numbers.insert(number.id, number.value);
                        }
                    })
                });
                numbers
            })
            .filter(|map| map.len() == 2)
            .filter_map(|map| map.into_values().reduce(|acc, v| acc * v))
            .collect()
    }
}

fn solve(input: Vec<&str>) -> String {
    let schematic = Schematic::parse(input);
    let ratios = schematic.compute_gear_ratios();
    println!("ratios: {:?}", ratios);
    let ratio = ratios.into_iter().reduce(|acc, v| acc + v).unwrap();
    ratio.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn day1_part1() {
        let input: Vec<_> = include_str!("part1-test.data").lines().collect();
        assert_eq!(solve(input), "467835");
    }
}
