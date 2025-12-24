use std::collections::{HashSet, VecDeque};

#[derive(Debug, Default)]
struct Machine {
    // 0 pad left, only the bottom 10 bits are used
    indicator_lights: u16,
    button_wirings: Vec<u16>,
    joltage_reqs: Vec<u16>,
}

impl Machine {
    fn apply_button(indicator_lights: u16, button: u16) -> u16 {
        indicator_lights ^ button
    }

    fn solve(&self) -> usize {
        let mut cache = HashSet::<u16>::default();
        let mut to_visit = VecDeque::default();
        to_visit.push_front((0, 0));
        while let Some((lights, n_presses)) = to_visit.pop_front() {
            if lights == self.indicator_lights {
                return n_presses;
            }
            for button in &self.button_wirings {
                let next = Self::apply_button(lights, *button);
                if !cache.insert(next) {
                    continue;
                }
                to_visit.push_back((next, n_presses + 1));
            }
        }

        panic!("failed to find any solution");
    }
}

impl From<&str> for Machine {
    fn from(line: &str) -> Self {
        let splits = line.split_whitespace().collect::<Vec<_>>();
        let mut out = Self {
            indicator_lights: splits
                .first()
                .expect("indicator lights")
                .strip_prefix('[')
                .expect("leading [")
                .strip_suffix(']')
                .expect("trailing ]")
                .chars()
                .enumerate()
                .filter_map(|(ind, c)| match c {
                    '.' => None,
                    '#' => Some(1 << ind),
                    _ => panic!("invalid indicator light char: {c}"),
                })
                .sum(),
            ..Default::default()
        };

        splits
            .last()
            .expect("joltage requirements")
            .strip_prefix('{')
            .expect("leading {")
            .strip_suffix('}')
            .expect("trailing }")
            .split(',')
            .for_each(|num| out.joltage_reqs.push(num.parse().expect("valid number")));
        let n = splits.len();
        for split in &splits[1..(n - 1)] {
            out.button_wirings.push(
                split
                    .strip_prefix('(')
                    .expect("leading (")
                    .strip_suffix(')')
                    .expect("trailing )")
                    .split(',')
                    .map(|num| 1 << num.parse::<u16>().expect("valid number"))
                    .sum(),
            );
        }
        out
    }
}

struct Manual(Vec<Machine>);

impl From<&str> for Manual {
    fn from(data: &str) -> Self {
        Self(data.lines().map(Machine::from).collect())
    }
}

fn main() {
    let Manual(machines) = Manual::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );
    println!(
        "{}",
        machines
            .into_iter()
            .map(|machine| machine.solve())
            .sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use crate::Machine;

    #[test]
    fn test_apply_button() {
        assert_eq!(Machine::apply_button(0, 5), 5);
        assert_eq!(Machine::apply_button(5, 3), 6);
    }
}
