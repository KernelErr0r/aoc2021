use std::{error::Error, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let binary_numbers = reader.lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    // Numbers used to calculate the oxygen generator rate
    let mut oxygen_generator_numbers = binary_numbers.clone();
    
    for position in 0 .. binary_numbers[0].len() {
        if oxygen_generator_numbers.len() == 1 {
            break;
        }

        let mut zeroes = 0usize;
        let mut ones = 0usize;

        for gamma_number in &oxygen_generator_numbers {
            let bit = gamma_number.chars().nth(position).unwrap();

            match bit {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Found an invalid character in the binary number"),
            }
        }

        if zeroes > ones {
            oxygen_generator_numbers.retain(|gamma_number| gamma_number.chars().nth(position).unwrap() == '0');
        } else if zeroes <= ones {
            oxygen_generator_numbers.retain(|gamma_number| gamma_number.chars().nth(position).unwrap() == '1');
        }
    }

    // Numbers used to calculate the co2 scrubber rate
    let mut co2_scrubber_numbers = binary_numbers.clone();

    for position in 0 .. binary_numbers[0].len() {
        if co2_scrubber_numbers.len() == 1 {
            break;
        }

        let mut zeroes = 0usize;
        let mut ones = 0usize;

        for epsilon_number in &co2_scrubber_numbers {
            let bit = epsilon_number.chars().nth(position).unwrap();

            match bit {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Found an invalid character in the binary number"),
            }
        }

        if zeroes > ones {
            co2_scrubber_numbers.retain(|epsilon_number| epsilon_number.chars().nth(position).unwrap() == '1');
        } else if zeroes <= ones {
            co2_scrubber_numbers.retain(|epsilon_number| epsilon_number.chars().nth(position).unwrap() == '0');
        }
    }

    let oxygen_generator_rate = usize::from_str_radix(&oxygen_generator_numbers[0], 2)?;
    let co2_scrubber_rate = usize::from_str_radix(&co2_scrubber_numbers[0], 2)?;
    let life_support_rate = oxygen_generator_rate * co2_scrubber_rate;

    println!("{}", life_support_rate);

    Ok(())
}
