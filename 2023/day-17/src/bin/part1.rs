use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub fn main() {
    let input = include_str!("part1.data").lines();

    let map = Map::new(
        input
            .map(|s| {
                s.chars()
                    .filter_map(|c| c.to_digit(10).map(|n| n as usize))
                    .collect()
            })
            .collect(),
    );

    let initial_status = Status {
        x: 0,
        y: 0,
        straight_moves: 0,
        direction: None,
    };

    let target = (map.width - 1, map.height - 1);
    println!("Target: {:?}", target);
    let result = map.compute_optimal_path(initial_status, target);

    println!("Result: {:?}", result);
}

type Cache = HashMap<CacheKey, (usize, Vec<(usize, usize)>)>;

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<usize>>,
}

impl Map {
    fn new(map: Vec<Vec<usize>>) -> Self {
        let width = map[0].len();
        let height = map.len();
        Self { width, height, map }
    }

    fn get_heat_loss(&self, x: usize, y: usize) -> Option<&usize> {
        self.map.get(y).and_then(|row| row.get(x))
    }

    fn compute_optimal_path(&self, initial_status: Status, target: (usize, usize)) -> usize {
        let mut cache: Cache = HashMap::new();
        let mut visited: VecDeque<(usize, usize)> = VecDeque::new();

        self._compute_optimal_path(&initial_status, &target, &mut cache, &mut visited)
    }

    fn _compute_optimal_path(
        &self,
        status: &Status,
        target: &(usize, usize),
        cache: &mut Cache,
        visited: &mut VecDeque<(usize, usize)>,
    ) -> usize {
        let current_heat_loss = *self
            .get_heat_loss(status.x, status.y)
            .expect("could not get heat");

        // We've reached the target
        if &(status.x, status.y) == target {
            return current_heat_loss;
        }

        // use the cache
        let key = status.into();
        if let Some(&(result, _)) = cache.get(&key) {
            return result;
        }

        // mark as visited
        visited.push_back((status.x, status.y));

        // recurse
        let to_expand = self
            .adjacent_positions(status)
            .iter()
            .filter(|(x, y, dir)| {
                // filter if we spent 3 moves in a row going straight
                let straight_moves_spent =
                    status.straight_moves >= 3 && status.direction == Some(*dir);
                let was_visited = visited.contains(&(*x, *y));

                !straight_moves_spent && !was_visited
            })
            .map(|(x, y, dir)| Status {
                x: *x,
                y: *y,
                direction: Some(*dir),
                straight_moves: if status.direction == Some(*dir) {
                    status.straight_moves + 1
                } else {
                    0
                },
            })
            .collect::<Vec<_>>();

        let min_heat_loss = to_expand
            .iter()
            .map(|new_status| self._compute_optimal_path(&new_status, target, cache, visited))
            .min()
            .and_then(|min| min.checked_add(current_heat_loss))
            .unwrap_or(usize::MAX);

        visited.pop_back();
        cache.insert(key, (min_heat_loss, visited.iter().copied().collect()));
        min_heat_loss
    }

    fn adjacent_positions(&self, status: &Status) -> Vec<(usize, usize, Direction)> {
        let mut result = Vec::new();
        if status.x > 0 {
            result.push((status.x - 1, status.y, Direction::West));
        }
        if status.x < self.width - 1 {
            result.push((status.x + 1, status.y, Direction::East));
        }
        if status.y > 0 {
            result.push((status.x, status.y - 1, Direction::North));
        }
        if status.y < self.height - 1 {
            result.push((status.x, status.y + 1, Direction::South));
        }
        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::North => '↑',
            Direction::South => '↓',
            Direction::East => '→',
            Direction::West => '←',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    x: usize,
    y: usize,
    direction: Option<Direction>,
    straight_moves: usize,
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} {} {}",
            self.x,
            self.y,
            self.direction.map(|d| d.to_string()).unwrap_or("∅".into()),
            self.straight_moves
        )
    }
}

impl From<&Status> for CacheKey {
    fn from(status: &Status) -> Self {
        Self {
            x: status.x,
            y: status.y,
            straight_moves: status.straight_moves,
            direction: status.direction,
        }
    }
}

struct Status {
    x: usize,
    y: usize,
    straight_moves: usize,
    direction: Option<Direction>,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} {} {}",
            self.x,
            self.y,
            self.direction.map(|d| d.to_string()).unwrap_or("∅".into()),
            self.straight_moves,
        )
    }
}

impl AsRef<Status> for Status {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[cfg(test)]
mod tests {}
