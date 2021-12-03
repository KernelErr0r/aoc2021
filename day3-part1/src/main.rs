use std::{error::Error, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let binary_numbers = reader.lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for position in 0 .. binary_numbers[0].len() {
        let mut zeroes = 0usize;
        let mut ones = 0usize;

        for binary_number in &binary_numbers {
            let bit = binary_number.chars().nth(position).unwrap();

            match bit {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Found an invalid character in the binary number"),
            }
        }

        if zeroes > ones {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        } else if zeroes <= ones {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        }
    }

    let gamma_rate = usize::from_str_radix(&gamma_rate, 2)?;
    let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2)?;
    let power_consumption = gamma_rate * epsilon_rate;

    println!("{}", power_consumption);

    Ok(())
}
