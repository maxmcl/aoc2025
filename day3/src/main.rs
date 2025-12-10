use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Joltage = u64;
#[derive(Debug)]
struct Bank {
    batteries: Vec<Joltage>,
}

type BankPower = [Joltage; 12];

fn compute_power(bank_power: &BankPower) -> Joltage {
    bank_power
        .iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(p, value)| value * (10 as Joltage).pow(p as u32))
        .sum()
}

impl Bank {
    fn recurse(values: &[Joltage], mut bank_power: BankPower, best: &mut Joltage, mut pos: usize) {
        if values.is_empty() {
            return;
        }
        let first = values[0];
        let n1 = bank_power.len() - 1;
        for next_pos in 0..=pos {
            if n1 - next_pos > values.len() - 1 {
                continue;
            }
            if first > bank_power[next_pos] {
                bank_power[next_pos] = first;
                bank_power[next_pos + 1..].fill(0);
                if next_pos == n1 {
                    *best = std::cmp::max(*best, compute_power(&bank_power));
                }
                pos = std::cmp::min(next_pos + 1, n1);
                break;
            }
        }
        Bank::recurse(&values[1..], bank_power, best, pos)
    }

    fn get_largest_joltage(&self) -> Joltage {
        // Microoptimization
        let max = self.batteries[..self.batteries.len() - 12]
            .iter()
            .max()
            .expect(">= 1 battery");
        let pos = self
            .batteries
            .iter()
            .position(|battery| battery == max)
            .expect("1 battery matches max");
        let mut bank_power = BankPower::default();
        bank_power[0] = *max;

        let mut best = 0;
        Bank::recurse(&self.batteries[pos + 1..], bank_power, &mut best, 1);
        println!("{self:?} | {best}");
        best
    }
}

type FileLines = Lines<BufReader<File>>;
fn parse(lines: FileLines) -> impl Iterator<Item = Bank> {
    lines.into_iter().map(|line| {
        let line = line.expect("line is valid");
        Bank {
            batteries: line
                .chars()
                .map(|c| c.to_digit(10).expect("battery has a valid value") as Joltage)
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
        .sum::<Joltage>()
    );
}

#[cfg(test)]
mod tests {}
