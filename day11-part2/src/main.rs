use std::{error::Error, fs::File, io::{BufReader, BufRead}};

const HEIGHT: usize = 10;
const WIDTH: usize = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut octopus_power_levels = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .map(|character| format!("{}", character).parse())
                .collect()
        })
        .collect::<Result<Vec<Vec<usize>>, _>>()?;

    let mut steps = 1;

    loop {
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                octopus_power_levels[y][x] += 1;
            }
        }

        let mut keep_evaluating = true;
        let mut octopuses_to_reset = Vec::new();

        while keep_evaluating {
            keep_evaluating = false;

            for y in 0 .. HEIGHT {
                for x in 0 .. WIDTH {
                    if octopus_power_levels[y][x] > 9 {
                        octopus_power_levels[y][x] = 0;
                        octopuses_to_reset.push((x, y));

                        let adjacent_positions = adjacent_positions(&octopus_power_levels, x, y);
    
                        for (adjacent_x, adjacent_y) in adjacent_positions {
                            if octopus_power_levels[adjacent_y][adjacent_x] == 9 {
                                keep_evaluating = true;
                            }

                            octopus_power_levels[adjacent_y][adjacent_x] += 1;
                        }
                    }
                }
            }
        }

        for octopus_to_reset in octopuses_to_reset {
            octopus_power_levels[octopus_to_reset.1][octopus_to_reset.0] = 0;
        }

        let mut all_flashed = true;

        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                if octopus_power_levels[y][x] != 0 {
                    all_flashed = false;
                }
            }
        }

        if all_flashed {
            break;
        }

        steps += 1;
    }

    println!("{}", steps);

    Ok(())
}

fn adjacent_positions(power_levels: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut adjacent_positions = Vec::new(); 

    let mut xs = vec![x];

    if x > 0 {
        xs.push(x - 1);
    }

    if x < power_levels[0].len() - 1 {
        xs.push(x + 1);
    }

    let mut ys = vec![y];

    if y > 0 {
        ys.push(y - 1);
    }

    if y < power_levels.len() - 1 {
        ys.push(y + 1);
    }

    for current_x in &xs {
        for current_y in &ys {
            if *current_x == x && *current_y == y {
                continue;
            }

            adjacent_positions.push((*current_x, *current_y));
        }
    }

    adjacent_positions
}
