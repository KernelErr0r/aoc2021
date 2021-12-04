#![feature(decl_macro)]

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let numbers = lines.next().unwrap().map(|line| {
        line.split(",")
            .filter_map(|number| number.parse::<usize>().ok())
            .collect::<Vec<_>>()
    })?;

    if let Some(line) = lines.next() {
        line?;
    }

    let mut bingo_boards = Vec::new();

    loop {
        let mut bingo_board = Vec::new();

        for _ in 0..5 {
            bingo_board.push(
                lines
                    .next()
                    .unwrap()
                    .unwrap()
                    .split(' ')
                    .filter(|element| *element != "")
                    .filter_map(|number| number.parse::<usize>().ok())
                    .collect::<Vec<_>>(),
            );
        }

        bingo_boards.push(BingoBoard::new(&bingo_board));

        if let None = lines.next() {
            break;
        }
    }

    for number in numbers {
        for bingo_board in &mut bingo_boards {
            bingo_board.mark(number);

            if bingo_board.has_bingo() {
                let unmarked_numbers_sum: usize = bingo_board.get_unmarked().into_iter().sum();
                let score = number * unmarked_numbers_sum;

                println!("{}", score);

                return Ok(());
            }
        }
    }

    Ok(())
}

struct BingoBoard {
    contents: [[(usize, bool); 5]; 5],
}

impl BingoBoard {
    fn new(contents: &Vec<Vec<usize>>) -> BingoBoard {
        let mut board = BingoBoard {
            contents: [[(0usize, false); 5]; 5],
        };

        for y in 0..5 {
            for x in 0..5 {
                board.contents[y][x] = (contents[y][x], false);
            }
        }

        board
    }

    fn has_bingo(&self) -> bool {
        for y in 0..5 {
            for x in 0..5 {
                if !self.contents[y][x].1 {
                    break;
                }

                if x == 4 {
                    return true;
                }
            }
        }

        for x in 0..5 {
            for y in 0..5 {
                if !self.contents[y][x].1 {
                    break;
                }

                if y == 4 {
                    return true;
                }
            }
        }

        false
    }

    fn mark(&mut self, number: usize) {
        for y in 0..5 {
            for x in 0..5 {
                if self.contents[y][x].0 == number {
                    self.contents[y][x].1 = true;
                }
            }
        }
    }

    fn get_unmarked(&self) -> Vec<usize> {
        self.contents
            .iter()
            .flat_map(|row| row)
            .filter(|(_, marked)| !marked)
            .map(|(value, _)| value.clone())
            .collect()
    }
}
