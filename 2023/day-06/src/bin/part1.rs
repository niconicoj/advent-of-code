use std::ops::RangeInclusive;

pub fn main() {
    let input = vec![
        Race {
            time: 49.0,
            distance: 298.0,
        },
        Race {
            time: 78.0,
            distance: 1185.0,
        },
        Race {
            time: 79.0,
            distance: 1066.0,
        },
        Race {
            time: 80.0,
            distance: 1181.0,
        }
    ];

    let solution = input
        .iter()
        .map(|r| {
            let solutions = r.solve();
            solutions.end() - solutions.start() + 1
        })
        .reduce(|acc, x| acc * x)
        .expect("could not compute a solution");

    println!("solution : {}", solution);
}

struct Race {
    time: f32,
    distance: f32,
}

impl Race {
    pub fn solve(&self) -> RangeInclusive<isize> {
        let discriminant = self.discriminant();
        let solutions = [1.0, -1.0]
            .iter()
            .map(|s| ((-self.time) + s * discriminant) / (-2.0))
            .filter(|s| s.is_sign_positive())
            .collect::<Vec<_>>();

        println!("float solutions : {:?}", solutions);

        match solutions.len() {
            2 => {
                let low_bound = solutions[0].ceil() as isize;
                let high_bound = solutions[1].floor() as isize;
                low_bound..=high_bound
            }
            1 => {
                let bound = solutions[0].round() as isize;
                bound..=bound
            }
            _ => unreachable!("Impossible number of solution found"),
        }
    }

    fn discriminant(&self) -> f32 {
        (self.time.powf(2.0) - 4.0 * (self.distance + 1.0)).sqrt()
    }
}

fn solve(input: Vec<&str>) -> String {
    let race = Race {
        time: 7.0,
        distance: 9.0,
    };
    let solutions = race.solve();
    println!("{:?}", solutions);
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::Race;

    #[test]
    fn day1_part1() {
        let input = vec![
            Race {
                time: 7.0,
                distance: 9.0,
            },
            Race {
                time: 15.0,
                distance: 40.0,
            },
            Race {
                time: 30.0,
                distance: 200.0,
            },
        ];

        let solutions = input
            .iter()
            .map(|r| {
                let solutions = r.solve();
                solutions.end() - solutions.start() + 1
            })
            .collect::<Vec<_>>();

        assert_eq!(solutions, vec![]);
    }
}
