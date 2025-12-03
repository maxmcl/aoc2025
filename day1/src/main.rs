use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct Direction(i16);

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "L" => Direction(-1),
            "R" => Direction(1),
            _ => panic!("Invalid direction: {c}"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i16,
}

type FileLines = Lines<BufReader<File>>;
fn parse(lines: FileLines) -> impl Iterator<Item = Instruction> {
    lines.into_iter().map(|line| {
        let line = line.expect("line is valid");
        Instruction {
            direction: Direction::from(&line[0..1]),
            value: line[1..].parse().expect("line ends with valid i16"),
        }
    })
}

fn main() {
    let mut count = 0;
    let mut dial = 50;
    const MAX: i16 = 100;
    for instruction in parse(
        BufReader::new(
            File::open(std::env::args().nth(1).expect("filename")).expect("file exists"),
        )
        .lines(),
    ) {
        println!("{dial} => {instruction:?}");
        dial = (dial + (instruction.direction.0 * instruction.value)).rem_euclid(MAX);
        if dial == 0 {
            count += 1;
        }
        println!("{dial}");
    }

    println!("{count}");
}
