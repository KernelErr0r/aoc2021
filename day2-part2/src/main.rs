use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut horizontal = 0isize;
    let mut depth = 0isize;
    let mut aim = 0isize;

    reader
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();
            let direction = String::from(split[0]);
            let amount = split[1].parse::<isize>().unwrap();

            (direction, amount)
        })
        .for_each(|(direction, amount)| match &*direction {
            "up" => aim -= amount,
            "down" => aim += amount,
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            }
            _ => unreachable!(),
        });

    println!("{}", horizontal * depth);

    Ok(())
}
