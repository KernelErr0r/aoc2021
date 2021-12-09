use std::{fs::File, io::{BufReader, BufRead}, error::Error, fmt::{self, Display, Formatter}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let result: usize = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut split = line.split('|');

            let unique_signal_patterns = split.next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|unique_signal_pattern| {
                    unique_signal_pattern.chars()
                        .map(Segment::from)
                        .collect()
                })
                .collect::<Vec<Vec<Segment>>>();

            let digit_output_values = split.next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|digit_output_value| {
                    digit_output_value.chars()
                        .map(Segment::from)
                        .collect()
                })
                .collect::<Vec<Vec<Segment>>>();

            digit_output_values.iter()
                .map(|digit_output_value| signal_to_possible_digits(&digit_output_value))
                .filter(|possible_digits| possible_digits.len() == 1)
                .count()
        })
        .sum();

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Display for Segment {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", match self {
            Segment::A => "a",
            Segment::B => "b",
            Segment::C => "c",
            Segment::D => "d",
            Segment::E => "e",
            Segment::F => "f",
            Segment::G => "g",
        })
    }
}

impl From<char> for Segment {
    fn from(character: char) -> Self {
        match character {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!(),
        }
    }
}

fn signal_to_possible_digits(signal: &Vec<Segment>) -> Vec<i32> {
    match signal.len() {
        2 => vec![1],
        3 => vec![7],
        4 => vec![4],
        5 => vec![2, 3, 5],
        6 => vec![0, 6, 9],
        7 => vec![8],
        _ => panic!(),
    }
}

fn correct_segments_for_number(number: i32) -> Vec<Segment> {
    match number {
        0 => vec![Segment::A, Segment::B, Segment::C, Segment::E, Segment::F, Segment::G],
        1 => vec![Segment::C, Segment::F],
        2 => vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G],
        3 => vec![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G],
        4 => vec![Segment::B, Segment::C, Segment::D, Segment::F],
        5 => vec![Segment::A, Segment::B, Segment::D, Segment::F, Segment::G],
        6 => vec![Segment::A, Segment::B, Segment::D, Segment::E, Segment::F, Segment::G],
        7 => vec![Segment::A, Segment::C, Segment::F],
        8 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G],
        9 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G],
        _ => panic!(),
    }
}
