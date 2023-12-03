use std::collections::HashSet;

pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

type Coord = (usize, usize);

#[derive(Debug)]
pub struct Schematic {
    symbols: HashSet<Coord>,
    numbers: Vec<(usize, Coord)>,
}

impl Schematic {
    pub fn new() -> Self {
        Self {
            symbols: HashSet::new(),
            numbers: Vec::new(),
        }
    }

    pub fn parse(input: Vec<&str>) -> Schematic {
        let mut schematic = Self::new();
        for (y, line) in input.iter().enumerate() {
            let mut line_iter = line.chars().enumerate().peekable();
            while let Some((x, c)) = line_iter.next() {
                match c {
                    '.' => continue,
                    c if c.is_numeric() => {
                        let mut number = String::from(c);
                        while let Some((_, c)) = line_iter.peek() {
                            if c.is_numeric() {
                                number.push(line_iter.next().unwrap().1);
                            } else {
                                break;
                            }
                        }
                        schematic.add_number(number.parse::<usize>().unwrap(), (x, y));
                    }
                    _ => schematic.add_symbol((x, y)),
                }
            }
        }
        schematic
    }

    fn add_symbol(&mut self, c: Coord) {
        println!("add symbol at {:?}", c);
        // this assumes that there are no symbols on the edges
        for x in (c.0 - 1)..=(c.0 + 1) {
            for y in (c.1 - 1)..=(c.1 + 1) {
                self.symbols.insert((x, y));
            }
        }
    }

    fn add_number(&mut self, number: usize, c: Coord) {
        self.numbers.push((number, c));
    }

    pub fn numbers_on_symbols(&self) -> Vec<&(usize, Coord)> {
        self.numbers
            .iter()
            .filter(|(number, c)| {
                let len = (number.checked_ilog10().unwrap_or(0) + 1) as usize;
                (c.0..(c.0 + len)).any(|x| self.symbols.contains(&(x, c.1)))
            })
            .collect()
    }
}

fn solve(input: Vec<&str>) -> String {
    let schematic = Schematic::parse(input);
    let sum = schematic
        .numbers_on_symbols()
        .iter()
        .fold(0, |acc, (n, _)| acc + n);
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::solve;

    #[test]
    fn day1_part1() {
        let input: Vec<_> = include_str!("part1-test.data").lines().collect();
        assert_eq!(solve(input), "4361");
    }
}
