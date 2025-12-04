use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
}

const MAX: i16 = 100;
impl Direction {
    fn shift(self, value: i16) -> i16 {
        match self {
            Direction::Left => MAX - value,
            Direction::Right => value,
        }
    }
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction: {c}"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i16,
}

impl Instruction {
    fn new(direction: Direction, value: i16) -> Self {
        if value <= 0 {
            panic!("value = {value} must be >= 0");
        }
        Self { direction, value }
    }
}

type FileLines = Lines<BufReader<File>>;
fn parse(lines: FileLines) -> impl Iterator<Item = Instruction> {
    lines.into_iter().map(|line| {
        let line = line.expect("line is valid");
        Instruction::new(
            Direction::from(&line[0..1]),
            line[1..].parse().expect("line ends with valid i16"),
        )
    })
}

fn solve(mut dial: i16, instructions: impl Iterator<Item = Instruction>) -> i16 {
    let mut count = 0;
    for instruction in instructions {
        let mut next = instruction.direction.shift(dial) + instruction.value;
        count += ((next as f64 / MAX as f64).floor() as i16).abs();
        if instruction.direction == Direction::Left && dial == 0 {
            count -= 1;
        }
        next = next.rem_euclid(MAX);
        if next != 0 {
            dial = instruction.direction.shift(next);
        } else {
            dial = 0;
        }
    }

    count
}

fn main() {
    println!(
        "{}",
        solve(
            50,
            parse(
                BufReader::new(
                    File::open(std::env::args().nth(1).expect("filename")).expect("file exists"),
                )
                .lines()
            )
        )
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_turn_right() {
        assert_eq!(
            solve(95, vec![Instruction::new(Direction::Right, 60)].into_iter()),
            1
        );
        assert_eq!(
            solve(0, vec![Instruction::new(Direction::Right, 100)].into_iter()),
            1
        );
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Right, 50)].into_iter()),
            1
        );
        assert_eq!(
            solve(
                50,
                vec![Instruction::new(Direction::Right, 100)].into_iter()
            ),
            1
        );
        assert_eq!(
            solve(
                0,
                vec![Instruction::new(Direction::Right, 1000)].into_iter()
            ),
            10
        );
        assert_eq!(
            solve(
                50,
                vec![Instruction::new(Direction::Right, 1000)].into_iter()
            ),
            10
        );
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Right, 5)].into_iter()),
            0
        );
        assert_eq!(
            solve(0, vec![Instruction::new(Direction::Right, 5)].into_iter()),
            0
        );
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Right, 1)].into_iter()),
            0
        );
        assert_eq!(
            solve(0, vec![Instruction::new(Direction::Right, 1)].into_iter()),
            0
        );
    }

    #[test]
    fn test_turn_left() {
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Left, 51)].into_iter()),
            1
        );
        assert_eq!(
            solve(0, vec![Instruction::new(Direction::Left, 1)].into_iter()),
            0
        );
        assert_eq!(
            solve(0, vec![Instruction::new(Direction::Left, 5)].into_iter()),
            0
        );
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Left, 5)].into_iter()),
            0
        );
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Left, 1)].into_iter()),
            0
        );
        assert_eq!(
            solve(0, vec![Instruction::new(Direction::Left, 100)].into_iter()),
            1
        );
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Left, 50)].into_iter()),
            1
        );
        assert_eq!(
            solve(0, vec![Instruction::new(Direction::Left, 1000)].into_iter()),
            10
        );
        assert_eq!(
            solve(50, vec![Instruction::new(Direction::Left, 100)].into_iter()),
            1
        );
        assert_eq!(
            solve(
                50,
                vec![Instruction::new(Direction::Left, 1000)].into_iter()
            ),
            10
        );
    }

    #[test]
    fn test_combine_one() {
        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Left, 50),
                    Instruction::new(Direction::Right, 50)
                ]
                .into_iter()
            ),
            1
        );

        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Left, 50),
                    Instruction::new(Direction::Left, 50)
                ]
                .into_iter()
            ),
            1
        );

        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Right, 50),
                    Instruction::new(Direction::Left, 50)
                ]
                .into_iter()
            ),
            1
        );

        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Right, 50),
                    Instruction::new(Direction::Right, 50)
                ]
                .into_iter()
            ),
            1
        );
    }

    #[test]
    fn test_combine_two() {
        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Left, 150),
                    Instruction::new(Direction::Left, 50)
                ]
                .into_iter()
            ),
            2
        );

        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Left, 150),
                    Instruction::new(Direction::Left, 50)
                ]
                .into_iter()
            ),
            2
        );

        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Right, 150),
                    Instruction::new(Direction::Left, 50)
                ]
                .into_iter()
            ),
            2
        );

        assert_eq!(
            solve(
                50,
                vec![
                    Instruction::new(Direction::Right, 150),
                    Instruction::new(Direction::Right, 50)
                ]
                .into_iter()
            ),
            2
        );
    }
}
