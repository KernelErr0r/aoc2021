#![feature(try_blocks)]

use std::{fs::File, io::{BufReader, BufRead}, error::Error, cmp};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let points = reader.lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut split = line.split("->").take(2);

            let start = split.next()
                .map(|split| {
                    split.trim()
                        .split(",")
                        .take(2)
                        .map(|number| number.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .map(|result| result.ok())
                .flatten()
                .map(|values| Vector2::new(values[0], values[1]));

            let end = split.next()
                .map(|split| {
                    split.trim()
                        .split(",")
                        .take(2)
                        .map(|number| number.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .map(|result| result.ok())
                .flatten()
                .map(|values| Vector2::new(values[0], values[1]));

            try { 
                Line::new(start?, end?) 
            }
        })
        .collect::<Option<Vec<_>>>()
        .unwrap()
        .iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .flat_map(|line| line.get_points())
        .collect::<Vec<Vector2>>();

    let max_x = points.iter().map(|line| line.x).max().unwrap();
    let max_y = points.iter().map(|line| line.y).max().unwrap();

    let mut board = vec![vec![0; max_x + 1]; max_y + 1];

    for point in points {
        board[point.y][point.x] += 1;
    }

    let overlapping_points = board.iter()
        .flat_map(|row| row)
        .filter(|element| **element > 1)
        .count();

    println!("{}", overlapping_points);

    Ok(())
}

struct Line {
    start: Vector2,
    end: Vector2,
}

impl Line {
    fn new(start: Vector2, end: Vector2) -> Self {
        Self {
            start,
            end
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.x != self.end.x && self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x && self.start.y != self.end.y
    }

    fn get_points(&self) -> Vec<Vector2> {
        let mut results = Vec::new();

        let start_x = cmp::min(self.start.x, self.end.x);
        let end_x = cmp::max(self.start.x, self.end.x);
        let start_y = cmp::min(self.start.y, self.end.y);
        let end_y = cmp::max(self.start.y, self.end.y);

        if self.is_vertical() {
            for y in start_y ..= end_y {
                results.push(Vector2::new(self.start.x, y));
            }
        } else if self.is_horizontal() {
            for x in start_x ..= end_x {
                results.push(Vector2::new(x, self.start.y));
            }
        } else if self.start.x != self.end.x && self.start.y != self.end.y {
            for x in start_x ..= end_x {
                for y in start_y ..= end_y {
                    results.push(Vector2::new(x, y));
                }
            }
        }

        results
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vector2 {
    x: usize,
    y: usize,
}

impl Vector2 {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_points_test() {
        assert_eq!(
            Line::new(Vector2::new(1, 1), Vector2::new(1, 3)).get_points(),
            vec![Vector2::new(1, 1), Vector2::new(1, 2), Vector2::new(1, 3)]
        );

        assert_eq!(
            Line::new(Vector2::new(9, 7), Vector2::new(7, 7)).get_points(),
            vec![Vector2::new(7, 7), Vector2::new(8, 7), Vector2::new(9, 7)]
        )
    }

    #[test]
    fn is_vertical_test() {
        assert!(Line::new(Vector2::new(0, 0), Vector2::new(0, 1)).is_vertical());
        assert!(!Line::new(Vector2::new(0, 0), Vector2::new(1, 0)).is_vertical());
    }

    #[test]
    fn is_horizontal_test() {
        assert!(Line::new(Vector2::new(0, 0), Vector2::new(1, 0)).is_horizontal());
        assert!(!Line::new(Vector2::new(0, 0), Vector2::new(0, 1)).is_horizontal());
    }
}