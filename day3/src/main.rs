use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Joltage = u32;
#[derive(Debug)]
struct Bank {
    batteries: Vec<Joltage>,
}

struct State {
    best: Joltage,
    first: Joltage,
    second: Joltage,
    handler: Handler,
}

type Handler = fn(state: &mut State, value: Joltage);

impl State {
    // Totally unecessary state machine that runs once, but whatever
    fn handle_first(&mut self, value: Joltage) {
        self.first = value;
        self.handler = State::handle_second;
    }

    fn handle_second(&mut self, value: Joltage) {
        if value > self.second {
            self.second = value;
            self.best = 10 * self.first + self.second;
        }
        if value > self.first {
            self.first = value;
            self.second = Joltage::MIN;
        }
    }
}

impl Bank {
    fn get_largest_joltage(&self) -> Joltage {
        let mut state = State {
            best: Joltage::MIN,
            first: Joltage::MIN,
            second: Joltage::MIN,
            handler: State::handle_first,
        };
        self.batteries.iter().copied().for_each(|value| {
            (state.handler)(&mut state, value);
        });
        println!("{}", state.best);
        state.best
    }
}

type FileLines = Lines<BufReader<File>>;
fn parse(lines: FileLines) -> impl Iterator<Item = Bank> {
    lines.into_iter().map(|line| {
        let line = line.expect("line is valid");
        Bank {
            batteries: line
                .chars()
                .map(|c| c.to_digit(10).expect("battery has a valid value"))
                .collect(),
        }
    })
}

fn main() {
    println!(
        "{}",
        parse(
            BufReader::new(
                File::open(std::env::args().nth(1).expect("filename")).expect("file exists"),
            )
            .lines()
        )
        .map(|bank| bank.get_largest_joltage())
        .sum::<u32>()
    );
}

#[cfg(test)]
mod tests {}
