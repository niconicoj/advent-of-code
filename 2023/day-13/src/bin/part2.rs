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
        .filter_map(|p| p.find_symmetry(true))
        .sum::<usize>();

    println!("result: {}", result);
}

struct Pattern {
    width: usize,
    pattern: Vec<usize>,
}

impl Pattern {
    fn find_symmetry(&self, clear_smudges: bool) -> Option<usize> {
        let v_symmetries = self.find_vertical_symmetries(false);
        let h_symmetries = self.find_horizontal_symmetries(false);
        let dirty_result = match (v_symmetries.get(0), h_symmetries.get(0)) {
            (Some(dirty_plan), _) => Some(*dirty_plan),
            (_, Some(dirty_plan)) => Some(*dirty_plan),
            _ => None,
        };

        if !clear_smudges {
            dirty_result
        } else {
            self.clean_smudges(dirty_result)
        }
    }

    fn clean_smudges(&self, dirty_result: Option<usize>) -> Option<usize> {
        println!("dirty result: {:?}", dirty_result);
        let mut clean_symmetries = self.find_vertical_symmetries(true);
        clean_symmetries.append(&mut self.find_horizontal_symmetries(true));

        println!("clean symmetries: {:?}", clean_symmetries);

        clean_symmetries
            .iter()
            .find(|&a| dirty_result.is_some_and(|b| a != &b))
            .copied()
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

    fn find_vertical_symmetries(&self, clear_smudges: bool) -> Vec<usize> {
        (1..self.width)
            .filter_map(|plan| {
                let mut cleaned = false;
                let is_sym = (0..plan)
                    .map(
                        |i| match (self.get_column(i), self.get_column(2 * plan - i - 1)) {
                            (Some(a), Some(b)) => {
                                if a ^ b == 0 {
                                    true
                                } else {
                                    if (a ^ b).count_ones() == 1
                                        && cleaned == false
                                        && clear_smudges
                                    {
                                        cleaned = true;
                                        true
                                    } else {
                                        false
                                    }
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
            .collect()
    }

    fn find_horizontal_symmetries(&self, clear_smudges: bool) -> Vec<usize> {
        (1..self.pattern.len())
            .filter_map(|plan| {
                let mut cleaned = false;
                let is_sym = (0..plan)
                    .map(
                        |i| match (self.get_row(i), self.get_row(2 * plan - i - 1)) {
                            (Some(a), Some(b)) => {
                                if a ^ b == 0 {
                                    true
                                } else {
                                    if (a ^ b).count_ones() == 1
                                        && cleaned == false
                                        && clear_smudges
                                    {
                                        println!("cleaning smudge");
                                        cleaned = true;
                                        true
                                    } else {
                                        false
                                    }
                                }
                            }
                            _ => true,
                        },
                    )
                    .reduce(|acc, v| acc && v);
                match is_sym {
                    Some(true) => Some(plan * 100),
                    _ => None,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pattern_find_horizontal_symmetry() {
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
        assert_eq!(pattern.find_horizontal_symmetries(false), vec![400]);
    }

    #[test]
    fn test_pattern_find_horizontal_symmetry_alt() {
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
        assert_eq!(pattern.find_horizontal_symmetries(false), vec![300]);
    }

    #[test]
    fn test_pattern_find_horizontal_symmetry_none() {
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
        assert_eq!(pattern.find_horizontal_symmetries(false), vec![]);
    }

    #[test]
    fn test_pattern_find_horizontal_symmetry_clear_smudge_alt() {
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
        assert_eq!(pattern.find_horizontal_symmetries(true), vec![300]);
    }

    #[test]
    fn test_pattern_find_vertical_symmetry_alt() {
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
        assert_eq!(pattern.find_vertical_symmetries(false), vec![5]);
    }

    #[test]
    fn test_pattern_find_vertical_symmetry_none() {
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
        assert_eq!(pattern.find_vertical_symmetries(false), vec![]);
    }

    #[test]
    fn test_pattern_find_vertical_symmetry_clear_smudge() {
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
        assert_eq!(pattern.find_vertical_symmetries(true), vec![]);
    }

    #[test]
    fn test_pattern_find_horizontal_symmetry_clear_smudge() {
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
        assert_eq!(pattern.find_horizontal_symmetries(true), vec![100, 400]);
    }

    #[test]
    fn test_pattern_find_symmetry_a() {
        let pattern = super::Pattern {
            width: 9,
            pattern: vec![
                0b101100110,
                0b100001001,
                0b001100111,
                0b111110110,
                0b111110110,
                0b001100111,
                0b100001001,
            ],
        };
        assert_eq!(pattern.find_symmetry(false), Some(400));
    }

    #[test]
    fn test_pattern_find_symmetry_a_clean_smudges() {
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
        assert_eq!(pattern.find_symmetry(true), Some(100));
    }

    #[test]
    fn test_pattern_find_symmetry_b() {
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
        assert_eq!(pattern.find_symmetry(false), Some(5));
    }

    #[test]
    fn test_pattern_find_symmetry_b_clean_smudges() {
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
        assert_eq!(pattern.find_symmetry(true), Some(300));
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
