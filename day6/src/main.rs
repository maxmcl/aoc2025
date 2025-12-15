#![feature(iter_map_windows)]
use std::collections::HashMap;

type Number = u64;
type Op = fn(Problem) -> Number;
#[derive(Debug)]
struct Problem {
    numbers: Vec<Number>,
    operation: Op,
}

impl Problem {
    fn fail(self) -> Number {
        panic!("operation not set")
    }

    fn add(self) -> Number {
        self.numbers.into_iter().sum()
    }

    fn multiplty(self) -> Number {
        self.numbers.into_iter().product()
    }
}

// Only allow 0-9 digits
type NumberBit = u8;
fn compute_number(bits: Vec<NumberBit>) -> Number {
    // E.g. [6, 2, 3] => 3*10**0 + 2*10**1 + 6*10**2
    bits.into_iter()
        .rev()
        .enumerate()
        .map(|(p, num)| (num as Number) * (10 as Number).pow(p as u32))
        .sum()
}

#[derive(Debug, Default)]
struct Problems(Vec<Problem>);

impl From<&str> for Problems {
    fn from(data: &str) -> Self {
        let lines = data.lines().collect::<Vec<_>>();
        let mut num_bitsmap: HashMap<usize, Vec<NumberBit>> = HashMap::default();
        for line in lines[..lines.len() - 1].iter() {
            line.bytes().enumerate().for_each(|(ind, c)| match c {
                b'0'..=b'9' => num_bitsmap.entry(ind).or_default().push(c - b'0'),
                b' ' => (),
                _ => panic!("invalid numbers line character: {c}"),
            })
        }

        let mut out = Self::default();
        lines
            .last()
            .expect(">1 line")
            .bytes()
            .enumerate()
            .filter_map(|(ind, c)| match c {
                b'+' => Some((ind, Problem::add as Op)),
                b'*' => Some((ind, Problem::multiplty as Op)),
                b' ' => None,
                _ => panic!("unexpected ops line character {c}"),
            })
            // To allow using .map_windows
            .chain(std::iter::once((lines[0].len(), Problem::fail as Op)))
            .map_windows(|[(start, op), (end, _)]| (*start, *end, *op))
            .for_each(|(start, end, operation)| {
                out.0.push(Problem {
                    numbers: (start..end)
                        .filter_map(|pos| num_bitsmap.remove(&pos).map(compute_number))
                        .collect(),
                    operation,
                });
            });

        out
    }
}

fn main() {
    let problems = Problems::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );
    println!(
        "{}",
        problems
            .0
            .into_iter()
            .map(|problem| {
                dbg!(&problem);
                let out = (problem.operation)(problem);
                dbg!(&out);
                out
            })
            .sum::<Number>()
    );
}

#[cfg(test)]
mod tests {
    use crate::compute_number;

    #[test]
    fn test_compute_number() {
        assert_eq!(compute_number(vec![6, 2, 3]), 623);
        assert_eq!(compute_number(vec![0]), 0);
    }
}
