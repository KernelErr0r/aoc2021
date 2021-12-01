use std::{fs::File, io::{self, BufRead, BufReader}};

fn main() -> Result<(), io::Error>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let numbers = reader.lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut previous_number = None;
    let mut increased = 0usize;

    for number in numbers {
        if let Some(previous_number) = previous_number {
            if number > previous_number {
                increased += 1;
            }
        }

        previous_number = Some(number);
    }

    println!("{}", increased);

    Ok(())
}
