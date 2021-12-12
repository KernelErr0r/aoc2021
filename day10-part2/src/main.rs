use std::{fs::File, io::{BufReader, BufRead, self}, iter::Peekable};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut scores = reader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| {
            match parse_chunks(&mut line.chars().into_iter().peekable()) {
                Ok(_) => true,
                Err(err) => match err {
                    ParsingError::UnexpectedEnd => true,
                    ParsingError::UnexpectedCharacter { character } => false,
                    ParsingError::MismatchedCharacter { expected_character, actual_character } => false,
                },
            }
        })
        .map(|line| {
            let missing_characters = get_missing_characters_for_chunks(&mut line.chars().into_iter().peekable());
            let mut score = 0;

            missing_characters.chars().for_each(|character| {
                score *= 5;

                score += match character {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                };
            });

            score
        })
        .collect::<Vec<usize>>();

    scores.sort_unstable();

    let score = scores[(scores.len() - 1) / 2];

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

fn get_missing_characters_for_chunks(characters: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut result = String::new();
    
    while let Some(_) = characters.peek() {
        result.push_str(&get_missing_characters_for_chunk(characters));
    }

    result
}

fn get_missing_characters_for_chunk(characters: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let starting_character = characters.next().unwrap();

    let ending_character = match starting_character {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => unreachable!(),
    };

    let mut result = String::new();

    loop {
        let character = characters.peek();

        match character {
            Some(character) => {
                if *character == ending_character {
                    characters.next().unwrap();
                    break;
                } else if STARTING_CHARACTERS.contains(character) {
                    result.push_str(&get_missing_characters_for_chunk(characters));
                }
            },
            None => {
                result.push(ending_character);
                break;
            },
        }
    }

    result
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
