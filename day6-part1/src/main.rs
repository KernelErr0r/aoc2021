use std::{fs::File, io::{BufReader, BufRead}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut fishes = reader.lines()
        .next()
        .unwrap()?
        .trim()
        .split(',')
        .map(|number| number.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    for _day in 1 ..= 80 {
        let mut new_fishes = 0;

        for fish in &mut fishes {
            if *fish == 0 {
                *fish = 6;
                new_fishes += 1;
            } else {
                *fish -= 1;
            }
        }

        for _fish_index in 0 .. new_fishes {
            fishes.push(8);
        }
    }

    println!("{}", fishes.len());

    Ok(())
}
