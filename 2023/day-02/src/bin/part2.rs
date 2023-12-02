use std::collections::HashMap;

pub fn main() {
    let input: Vec<_> = include_str!("part1.data").lines().collect();

    println!("solution : {}", solve(input));
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

type Bag = HashMap<Color, usize>;

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("invalid color"),
        }
    }
}

struct Game {
    id: usize,
    bag: Bag,
}

impl Game {
    pub fn power(&self) -> usize {
        self.bag.iter().fold(1, |acc, (_, count)| acc * count)
    }
}

impl Game {
    pub fn parse(input: &str) -> Self {
        let (header, content) = input.split_once(": ").expect("invalid game format");
        let id = header
            .split_once(' ')
            .expect("invalid game format")
            .1
            .parse::<usize>()
            .expect("invalid game format");

        let mut bag = Bag::new();
        content.split("; ").for_each(|set| {
            set.split(", ").for_each(|cubes| {
                let (count, color) = cubes.split_once(' ').expect("invalid game format");
                let count = count.parse::<usize>().expect("invalid game format");
                let color: Color = color.into();

                let v = bag.entry(color).or_insert(0);
                *v = count.max(*v);
            })
        });

        Game { id, bag }
    }
}

fn solve(input: Vec<&str>) -> String {
    let mut bag: Bag = HashMap::new();
    bag.insert(Color::Red, 12);
    bag.insert(Color::Green, 13);
    bag.insert(Color::Blue, 14);
    let result = input
        .iter()
        .map(|game| Game::parse(game))
        .fold(0, |acc, g| acc + g.power());

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn day2_part2() {
        let input: Vec<_> = include_str!("part1-test.data").lines().collect();
        assert_eq!(solve(input), "2286");
    }
}
