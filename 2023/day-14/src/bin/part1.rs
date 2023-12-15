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

    let result = columns
        .iter()
        .map(|column| compute_column(column.to_vec()))
        .sum::<usize>();

    println!("result: {}", result);
}

fn compute_column(column: Vec<char>) -> usize {
    let mut current_offset = column.len() + 1;
    column
        .iter()
        .enumerate()
        .filter_map(|(i, c)| match c {
            'O' => {
                current_offset -= 1;
                Some(current_offset)
            }
            '#' => {
                current_offset = column.len() - i;
                None
            }
            _ => None,
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {}
