use std::{fs::File, io::{BufReader, BufRead, self}, iter::Peekable};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let score: usize = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            match parse_chunks(&mut line.chars().into_iter().peekable()) {
                Ok(_) => 0,
                Err(err) => match err {
                    ParsingError::UnexpectedEnd => 0,
                    ParsingError::MismatchedCharacter { expected_character, actual_character } => match actual_character {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    }
                    _ => 0,
                },
            }
        })
        .sum();

    println!("{}", score);

    Ok(())
}

const STARTING_CHARACTERS: [char; 4] = ['{', '(', '[', '<'];
const ENDING_CHARACTERS: [char; 4] = ['}', ')', ']', '>'];

fn parse_chunks(characters: &mut Peekable<impl Iterator<Item = char>>) -> Result<(), ParsingError> {
    while let Some(_) = characters.peek() {
        parse_chunk(characters)?;
    }

    Ok(())
}

fn parse_chunk(characters: &mut Peekable<impl Iterator<Item = char>>) -> Result<(), ParsingError> {
    let starting_character = characters.next().unwrap();

    let ending_character = match starting_character {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => return Err(ParsingError::UnexpectedCharacter {
            character: starting_character
        }),
    };

    loop {
        let character = characters.peek();

        match character {
            Some(character) => {
                if *character == ending_character {
                    characters.next().unwrap();
                    break;
                } else if ENDING_CHARACTERS.contains(character) {
                    return Err(ParsingError::MismatchedCharacter {
                        expected_character: ending_character,
                        actual_character: character.clone(),
                    });
                } else if STARTING_CHARACTERS.contains(character) {
                    parse_chunk(characters)?;
                } else {
                    return Err(ParsingError::UnexpectedCharacter { 
                        character: character.clone(),
                    });
                }
            },
            None => return Err(ParsingError::UnexpectedEnd),
        }
    }

    Ok(())
}

#[derive(Debug)]
enum ParsingError {
    UnexpectedEnd,
    UnexpectedCharacter {
        character: char,
    },
    MismatchedCharacter {
        expected_character: char,
        actual_character: char,
    }
}
