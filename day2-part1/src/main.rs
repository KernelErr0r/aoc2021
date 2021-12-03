use std::{fs::File, io::{self, BufRead, BufReader}};

fn main() -> Result<(), io::Error>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut horizontal = 0isize;
    let mut depth = 0isize;

    reader.lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();
            let direction = split[0];
            let amount = split[1].parse::<isize>().unwrap();

            match direction {
                "forward" => (amount, 0),
                "up" => (0, -1 * amount),
                "down" => (0, amount),
                _ => panic!(),
            }
        })
        .for_each(|(current_horizontal, current_depth)| {
            horizontal += current_horizontal;
            depth += current_depth;
        });

    println!("{}", horizontal * depth);

    Ok(())
}
