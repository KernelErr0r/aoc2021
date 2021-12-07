use std::{fs::File, io::{BufReader, BufRead}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let positions = reader.lines()
        .filter_map(|line| line.ok())
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()?;

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let min_cost: usize = (min ..= max).into_iter()
        .map(|current_position| {
            positions.iter()
                .map(|position| {
                    if current_position > *position {
                        current_position - *position
                    } else {
                        *position - current_position
                    }
                })
                .sum()
        })
        .min()
        .unwrap();

    println!("{}", min_cost);

    Ok(())
}
