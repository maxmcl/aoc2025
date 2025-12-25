use std::collections::{HashSet, VecDeque};

#[derive(Debug, Default)]
struct Machine {
    button_wirings: Vec<Vec<usize>>,
    joltage_reqs: Vec<u16>,
}

impl Machine {
    fn apply_button(mut counts: Vec<u16>, button: &[usize]) -> Vec<u16> {
        button
            .iter()
            .for_each(|&press_index| counts[press_index] += 1);
        counts
    }

    fn solve(&self) -> usize {
        let mut to_visit = VecDeque::default();
        // TODO: proceed in reverse an subtract from joltage_reqs, is it faster?
        let init_counts = vec![0; self.joltage_reqs.len()];
        to_visit.push_front((init_counts.clone(), 0, 0));
        // TODO: efficient hashing?
        let mut cache = HashSet::<Vec<_>>::default();
        cache.insert(init_counts);

        while let Some((counts, index, n_presses)) = to_visit.pop_front() {
            let next_presses = n_presses + 1;
            for (button_ind, button) in self.button_wirings[index..].iter().enumerate() {
                let next = Self::apply_button(counts.clone(), button);
                if !cache.insert(next.clone()) {
                    continue;
                }
                {
                    // Check if next is a solution or an invalid candidate
                    let mut n_eq = 0;
                    for (count, exp_count) in next.iter().zip(&self.joltage_reqs) {
                        match count.cmp(exp_count) {
                            std::cmp::Ordering::Equal => n_eq += 1,
                            std::cmp::Ordering::Greater => continue,
                            _ => (),
                        }
                    }
                    if n_eq == self.joltage_reqs.len() {
                        return next_presses;
                    }
                }
                // Keep exploring candidate, limiting buttons from index + button_ind
                to_visit.push_back((next, index + button_ind, next_presses));
            }
        }

        panic!("failed to find any solution");
    }
}

impl From<&str> for Machine {
    fn from(line: &str) -> Self {
        let splits = line.split_whitespace().collect::<Vec<_>>();
        let mut out = Self::default();
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
                    .map(|num| num.parse().expect("valid number"))
                    .collect(),
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
