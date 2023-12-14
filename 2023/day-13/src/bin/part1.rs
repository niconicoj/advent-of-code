pub fn main() {
    let mut input = include_str!("part1.data").lines();

    let mut patterns = vec![];
    let mut pattern = vec![];
    let mut width = 0;

    while let Some(line) = input.next() {
        if line.is_empty() {
            patterns.push(Pattern {
                width,
                pattern: pattern.clone(),
            });
            pattern.clear();
            width = 0;
            continue;
        }

        width = line.len();
        let v = line
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("unexpected char"),
            })
            .fold(0, |acc, v| acc << 1 | v);
        pattern.push(v);
    }

    let result = patterns
        .iter()
        .filter_map(|p| p.find_symetry())
        .sum::<usize>();

    println!("result: {}", result);
}

struct Pattern {
    width: usize,
    pattern: Vec<usize>,
}

impl Pattern {
    fn find_symetry(&self) -> Option<usize> {
        match (self.find_vertical_symetry(), self.find_horizontal_symetry()) {
            (Some(plan), None) => Some(plan),
            (None, Some(plan)) => Some(plan * 100),
            _ => None,
        }
    }

    fn get_row(&self, row: usize) -> Option<usize> {
        self.pattern.get(row).copied()
    }

    fn get_column(&self, col: usize) -> Option<usize> {
        if col >= self.width {
            return None;
        }
        Some(
            self.pattern
                .iter()
                .map(|r| r >> (self.width - col - 1) & 1)
                .rev()
                .enumerate()
                .fold(0, |acc, (i, v)| acc | v << i),
        )
    }

    fn find_vertical_symetry(&self) -> Option<usize> {
        (1..self.width).find_map(|plan| {
            let is_sym = (0..plan)
                .map(
                    |i| match (self.get_column(i), self.get_column(2 * plan - i - 1)) {
                        (Some(a), Some(b)) => {
                            if a ^ b == 0 {
                                true
                            } else {
                                false
                            }
                        }
                        _ => true,
                    },
                )
                .reduce(|acc, v| acc && v);
            match is_sym {
                Some(true) => Some(plan),
                _ => None,
            }
        })
    }

    fn find_horizontal_symetry(&self) -> Option<usize> {
        (1..self.pattern.len()).find_map(|plan| {
            let is_sym = (0..plan)
                .map(
                    |i| match (self.get_row(i), self.get_row(2 * plan - i - 1)) {
                        (Some(a), Some(b)) => {
                            if a ^ b == 0 {
                                true
                            } else {
                                false
                            }
                        }
                        _ => true,
                    },
                )
                .reduce(|acc, v| acc && v);
            match is_sym {
                Some(true) => Some(plan),
                _ => None,
            }
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pattern_find_horizontal_symetry() {
        let pattern = super::Pattern {
            width: 3,
            pattern: vec![
                0b100011001,
                0b100001001,
                0b001100111,
                0b111110110,
                0b111110110,
                0b001100111,
                0b100001001,
            ],
        };
        assert_eq!(pattern.find_horizontal_symetry(), Some(4));
    }

    #[test]
    fn test_pattern_find_horizontal_symetry_alt() {
        let pattern = super::Pattern {
            width: 3,
            pattern: vec![
                0b100001001,
                0b001100111,
                0b111110110,
                0b111110110,
                0b001100111,
                0b100001001,
            ],
        };
        assert_eq!(pattern.find_horizontal_symetry(), Some(3));
    }

    #[test]
    fn test_pattern_find_horizontal_symetry_none() {
        let pattern = super::Pattern {
            width: 9,
            pattern: vec![
                0b101100110,
                0b001011010,
                0b110000001,
                0b110000001,
                0b001011010,
                0b001100110,
                0b101011010,
            ],
        };
        assert_eq!(pattern.find_horizontal_symetry(), None);
    }

    #[test]
    fn test_pattern_find_vertical_symetry_alt() {
        let pattern = super::Pattern {
            width: 9,
            pattern: vec![
                0b101100110,
                0b001011010,
                0b110000001,
                0b110000001,
                0b001011010,
                0b001100110,
                0b101011010,
            ],
        };
        assert_eq!(pattern.find_vertical_symetry(), Some(5));
    }

    #[test]
    fn test_pattern_find_vertical_symetry_none() {
        let pattern = super::Pattern {
            width: 9,
            pattern: vec![
                0b100001001,
                0b001100111,
                0b111110110,
                0b111110110,
                0b001100111,
                0b100001001,
            ],
        };
        assert_eq!(pattern.find_vertical_symetry(), None);
    }

    #[test]
    fn test_pattern_find_symetry_a() {
        let pattern = super::Pattern {
            width: 9,
            pattern: vec![
                0b100011001,
                0b100001001,
                0b001100111,
                0b111110110,
                0b111110110,
                0b001100111,
                0b100001001,
            ],
        };
        assert_eq!(pattern.find_symetry(), Some(400));
    }

    #[test]
    fn test_pattern_find_symetry_b() {
        let pattern = super::Pattern {
            width: 9,
            pattern: vec![
                0b101100110,
                0b001011010,
                0b110000001,
                0b110000001,
                0b001011010,
                0b001100110,
                0b101011010,
            ],
        };
        assert_eq!(pattern.find_symetry(), Some(5));
    }

    #[test]
    fn get_column_test() {
        let pattern = super::Pattern {
            width: 9,
            pattern: vec![
                0b101100110,
                0b001011010,
                0b110000001,
                0b110000001,
                0b001011010,
                0b001100110,
                0b101011010,
            ],
        };
        assert_eq!(pattern.get_column(0), Some(0b1011001));
        assert_eq!(pattern.get_column(1), Some(0b0011000));
        assert_eq!(pattern.get_column(8), Some(0b0011000));
        assert_eq!(pattern.get_column(9), None);
    }
}
