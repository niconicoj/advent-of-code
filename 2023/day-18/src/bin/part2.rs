pub fn main() {
    let input = include_str!("sample1").lines().take_while(|l| !l.is_empty());

    let mut vertices: Vec<(isize, isize)> = Vec::new();

    let mut current_position = (0, 0);
    vertices.push(current_position);

    input.for_each(|line| {
        /*
        let instruction = line
            .split_once(' ')
            .and_then(|(_, l)| l.split_once(' '))
            .unwrap()
            .1
            .trim_matches(['(', '#', ')']);

        let (distance, direction) = instruction.split_at(instruction.len() - 1);
        let distance = isize::from_str_radix(distance, 16).unwrap();
        */

        let (direction, rest) = line.split_once(' ').expect("bad format");
        let (distance, _) = rest.split_once(' ').expect("bad format");

        let distance = distance.parse::<isize>().expect("bad format") + 1;


        match direction {
            // Up
            "U" => {
                current_position = (current_position.0, current_position.1 + distance);
                vertices.push(current_position);
            }
            // Down
            "D" => {
                current_position = (current_position.0, current_position.1 - distance);
                vertices.push(current_position);
            }
            // Left
            "L" => {
                current_position = (current_position.0 - distance, current_position.1);
                vertices.push(current_position);
            }
            // Right
            "R" => {
                current_position = (current_position.0 + distance, current_position.1);
                vertices.push(current_position);
            }
            _ => panic!("bad format"),
        };
    });

    println!("vertices : {:?}", vertices);
    println!("vertices len : {}", vertices.len());

    let mut area = 0;

    vertices.windows(2).for_each(|slice| {
        if let &[v1, v2] = slice {
            area += v1.0 * v2.1;
            area -= v1.1 * v2.0;
        }
    });

    println!("area: {}", area / 2);
}
#[cfg(test)]
mod tests {}
