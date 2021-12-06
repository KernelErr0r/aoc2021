use std::{fs::File, io::{BufReader, BufRead}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let fishes = reader.lines()
        .next()
        .unwrap()?
        .trim()
        .split(',')
        .map(|number| number.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut fish_groups = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];

    for i in 0 .. 9 {
        fish_groups[i] = fishes.iter()
            .filter(|fish| **fish == i)
            .count();
    }

    for _day in 1 ..= 256 {
        let new_fish = fish_groups[0];

        for i in 0 .. 8 {
            fish_groups[i] = fish_groups[i + 1];
        }

        fish_groups[6] += new_fish;  
        fish_groups[8] = new_fish;
    }

    let total_fish: usize = fish_groups.iter().sum();

    println!("{}", total_fish);

    Ok(())
}
