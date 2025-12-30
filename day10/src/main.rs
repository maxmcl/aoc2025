use std::collections::HashMap;
use std::collections::hash_map::Entry::Vacant;

#[derive(Debug, Default)]
struct Machine {
    button_wirings: Vec<Vec<usize>>,
    joltage_reqs: Vec<u16>,
}

#[derive(Debug)]
struct ButtonPresses {
    joltages: Vec<u16>,
    n: usize,
}

fn _combinations(mut curr: Vec<usize>, ind: usize, n: usize, k: usize, out: &mut Vec<Vec<usize>>) {
    let curr_len = curr.len();
    if curr_len == k {
        out.push(curr);
        return;
    }
    for i in ind..(n - k + 1 + curr_len) {
        curr.push(i);
        _combinations(curr.clone(), i + 1, n, k, out);
        curr.pop();
    }
}

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    assert!(n >= k);
    // TODO: actual combination formula to get right capacity
    let mut out = Vec::with_capacity(n * k);
    _combinations(vec![], 0, n, k, &mut out);
    out
}

fn apply_button(joltages: &mut [u16], button: &[usize]) {
    button
        .iter()
        .for_each(|&press_index| joltages[press_index] += 1);
}

// bit 'i' set to 1 when count 'i' is odd
type OddMask = u16;
type OddMap = HashMap<OddMask, Vec<ButtonPresses>>;
fn build_even_map(button_wirings: &[Vec<usize>], size: usize) -> OddMap {
    // Max joltages length is 10
    const N_BIT_VALUES: usize = 1 << 10;
    // Not sure if helpful vs keeping all combinations
    let mut map_of_maps = HashMap::<OddMask, HashMap<Vec<u16>, usize>>::with_capacity(N_BIT_VALUES);
    let n_buttons = button_wirings.len();
    for n_pressed in 0..=n_buttons {
        for indices in combinations(n_buttons, n_pressed) {
            let mut joltages = vec![0; size];
            indices
                .into_iter()
                .for_each(|ind| apply_button(&mut joltages, &button_wirings[ind]));
            if let Vacant(entry) = map_of_maps
                .entry(get_odd_mask(&joltages))
                .or_default()
                .entry(joltages)
            {
                entry.insert(n_pressed);
            } // else: ignored, we only keep min n_pressed
        }
    }
    map_of_maps
        .into_iter()
        .map(|(mask, map)| {
            (
                mask,
                map.into_iter()
                    .map(|(joltages, n)| ButtonPresses { joltages, n })
                    .collect(),
            )
        })
        .collect()
}

fn get_odd_mask(values: &[u16]) -> OddMask {
    values
        .iter()
        .rev()
        .enumerate()
        .map(|(ind, &value)| if value % 2 != 0 { 1 << ind } else { 0 })
        .sum()
}

type Cache = HashMap<Vec<u16>, Option<usize>>;
impl Machine {
    fn maybe_subtract(mut joltages: Vec<u16>, other: &[u16]) -> Option<Vec<u16>> {
        debug_assert!(joltages.len() == other.len());
        for (v1, v2) in joltages.iter_mut().zip(other) {
            if *v1 < *v2 {
                return None;
            }
            *v1 -= v2;
        }
        Some(joltages)
    }

    fn recurse(joltage_reqs: Vec<u16>, even_map: &OddMap, cache: &mut Cache) -> Option<usize> {
        if let Some(value) = cache.get(&joltage_reqs) {
            return *value;
        }

        // Get the patterns that can convert joltage_reqs to a sequence of even numbers
        let mut min = None;
        let Some(all_button_presses) = even_map.get(&get_odd_mask(&joltage_reqs)) else {
            // No solution exists for joltage_reqs
            return min;
        };

        for button_presses in all_button_presses {
            /* Subtracting from the joltages will produce a sequence of even joltages.
               For joltages J obtained from P presses, pressing P twice would yield
               joltages 2*J
               <=>
               if J' = 2*J, then solving J'/2 = J should yield counts P and thus
               J' can be obtained by pressing 2*P
            */
            let n_presses = Self::maybe_subtract(joltage_reqs.clone(), &button_presses.joltages)
                .map(|mut joltages| {
                    joltages.iter_mut().for_each(|count| *count /= 2);
                    joltages
                })
                .and_then(|next| Self::recurse(next, even_map, cache))
                .map(|n_presses| 2 * n_presses + button_presses.n);

            match (min, n_presses) {
                (_, None) => (),
                (None, Some(n)) => min = Some(n),
                (Some(curr), Some(n)) => min = Some(std::cmp::min(curr, n)),
            }
        }
        cache.insert(joltage_reqs, min);
        min
    }

    fn solve(self) -> usize {
        let mut cache = Cache::new();
        cache.insert(vec![0; self.joltage_reqs.len()], Some(0));
        let map = build_even_map(&self.button_wirings, self.joltage_reqs.len());
        Self::recurse(self.joltage_reqs, &map, &mut cache).expect("solution must exist")
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

#[cfg(test)]
mod tests {
    use crate::{combinations, get_odd_mask};
    #[test]
    fn test_combinations() {
        assert_eq!(combinations(0, 0), vec![vec![]]);
        assert_eq!(combinations(10, 0), vec![vec![]]);
        assert_eq!(
            combinations(4, 1),
            vec![vec![0], vec![1], vec![2], vec![3],]
        );
        assert_eq!(
            combinations(4, 2),
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                //
                vec![1, 2],
                vec![1, 3],
                //
                vec![2, 3]
            ]
        );
        assert_eq!(
            combinations(5, 3),
            vec![
                vec![0, 1, 2],
                vec![0, 1, 3],
                vec![0, 1, 4],
                vec![0, 2, 3],
                vec![0, 2, 4],
                vec![0, 3, 4],
                //
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 3, 4],
                //
                vec![2, 3, 4],
            ]
        );
    }

    #[test]
    fn test_get_odd_mask() {
        assert_eq!(get_odd_mask(&[]), 0b0);
        assert_eq!(get_odd_mask(&[3, 7, 1, 10, 5, 100, 2]), 0b1110100);
    }
}
