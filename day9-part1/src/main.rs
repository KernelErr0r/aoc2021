use std::{fs::File, io::{BufReader, BufRead, self}};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let map = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .map(|character| format!("{}", character))
                .filter_map(|character| character.parse::<usize>().ok())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut risk = 0;

    for y in 0 .. map.len() {
        for x in 0 .. map[y].len() {
            if is_low_point(&map, (x, y)) {
                risk += map[y][x] + 1;
            }
        }
    }

    println!("{}", risk);

    Ok(())
}

fn is_low_point(map: &Vec<Vec<usize>>, point: (usize, usize)) -> bool {
    get_adjacent_locations(map, point).into_iter().all(|adjacent_point| adjacent_point > map[point.1][point.0])
}

fn get_adjacent_locations(map: &Vec<Vec<usize>>, point: (usize, usize)) -> Vec<usize> {
    let (x, y) = point;
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut adjacent_locations = Vec::new();

    if y != 0 {
        adjacent_locations.push(map[y - 1][x]);
    }

    if x != 0 {
        adjacent_locations.push(map[y][x - 1]);
    }

    if y != max_y {
        adjacent_locations.push(map[y + 1][x]);
    }

    if x != max_x {
        adjacent_locations.push(map[y][x + 1]);
    }

    adjacent_locations
}
