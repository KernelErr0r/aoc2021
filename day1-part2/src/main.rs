use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Add,
};

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let numbers = reader
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut increased = 0usize;

    for index in 0..numbers.len() - 3 {
        let first_window = (numbers[index], numbers[index + 1], numbers[index + 2]);
        let second_window = (numbers[index + 1], numbers[index + 2], numbers[index + 3]);

        if sum(first_window) < sum(second_window) {
            increased += 1;
        }
    }

    println!("{}", increased);

    Ok(())
}

fn sum<T: Add<Output = T>>(tuple: (T, T, T)) -> T {
    tuple.0 + tuple.1 + tuple.2
}
