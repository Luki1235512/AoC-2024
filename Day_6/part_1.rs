use std::fs;

fn parse_map(input: &str) -> (usize, Vec<bool>, usize) {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let map = input.chars().filter(|&c| c != '\n').map(|c| match c {
        '.' | '^' => false,
        '#' => true,
        _ => panic!(),
    }).collect::<Vec<_>>();
    let start = input.chars().filter(|&c| c != '\n').position(|c| c == '^').unwrap();
    (width, map, start)
}

fn trace_path(width: usize, map: &[bool], start: usize, start_heading: usize) -> u16 {
    let mut pos = start;
    let mut heading = start_heading;
    let mut visited = vec![false; map.len()];

    let mut visited_in_direction = vec![[false; 4]; map.len()];
    let mut visited_count = 0;

    loop {
        if visited_in_direction[pos][heading] {
            break true;
        }

        if !visited[pos] {
            visited[pos] = true;
            visited_count += 1;
        }
        visited_in_direction[pos][heading] = true;

        let outside_map = match heading {
            0 => pos < width,
            1 => pos % width == width - 1,
            2 => pos >= map.len() - width,
            3 => pos % width == 0,
            _ => panic!(),
        };

        if outside_map {
            break false;
        }

        let next_pos = match heading {
            0 => pos - width,
            1 => pos + 1,
            2 => pos + width,
            3 => pos - 1,
            _ => panic!(),
        };

        if map[next_pos] {
            heading = (heading + 1) % 4;
        } else {
            pos = next_pos;
        }
    };

    visited_count
}

fn main() {
    let input = fs::read_to_string("Day_6/input.txt").unwrap();

    let (width, map, start) = parse_map(&input);
    let visited_count = trace_path(width, &map, start, 0);

    println!("{}", visited_count);
}