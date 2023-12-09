pub fn main() {
    let input = include_str!("part1.data").lines().collect();

    let result = solve(input);

    println!("{}", result);
}

fn solve(input: Vec<&str>) -> String {
    let results = input
        .iter()
        .map(|line| {
            let sensor_readings = line
                .split(" ")
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            predict_next_value(sensor_readings)
        })
        .collect::<Vec<_>>();

    println!("results: {:?}", results);

    results.iter().sum::<isize>().to_string()
}

fn predict_next_value(sensor_readings: Vec<isize>) -> isize {
    let diffs = sensor_readings
        .windows(2)
        .map(|w| match w {
            [a, b] => b - a,
            _ => unreachable!("Unexpected window size"),
        })
        .collect::<Vec<_>>();
    let first = *sensor_readings.first().expect("readings was empty");
    if diffs.iter().all(|&v| v == 0) {
        first
    } else {
        return first - predict_next_value(diffs);
    }
}

#[cfg(test)]
mod tests {}
