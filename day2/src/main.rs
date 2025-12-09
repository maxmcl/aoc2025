use std::collections::HashSet;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Debug)]
struct Group {
    // TODO: pick better names
    value: u64,
    base: u64,
    n: u32,
    next_base: u64,
    size: u32,
    group_size: u32,
}

impl Group {
    fn from(value: u64, size: u32) -> Self {
        let n_digits = Group::get_n_digits(value);
        let mut out = if n_digits.is_multiple_of(size) {
            let n = n_digits / size;
            Self {
                value: 0,
                n,
                base: value / 10u64.pow(n_digits - n),
                next_base: 10u64.pow(n),
                size: n_digits,
                group_size: size,
            }
        } else {
            let n = n_digits.div_ceil(size);
            let next_base = (10u64).pow(n);
            Self {
                value: 0,
                base: next_base / 10,
                next_base,
                n,
                size: n * size,
                group_size: size,
            }
        };
        out.compute_value();
        out
    }

    fn next(&mut self) {
        self.base += 1;
        if self.base == self.next_base {
            // Skip odd numbers
            self.n += 1;
            self.next_base *= 10;
            self.size = self.group_size * self.n;
        }
        self.compute_value();
    }

    fn compute_value(&mut self) {
        self.value = self.base
            * (0..self.size)
                .step_by(self.n as usize)
                .map(|p| 10_u64.pow(p))
                .sum::<u64>()
    }

    fn get_n_digits(mut value: u64) -> u32 {
        let mut out = 0;
        while value > 0 {
            value /= 10;
            out += 1;
        }
        out
    }
}

impl Range {
    fn sum_invalid_ids(&self) -> u64 {
        let mut invalid_ids = HashSet::default();
        for size in 2..=Group::get_n_digits(self.end) {
            self.get_invalid_ids_for_group(size, &mut invalid_ids);
        }
        invalid_ids.into_iter().sum()
    }

    fn get_invalid_ids_for_group(&self, size: u32, invalid_ids: &mut HashSet<u64>) {
        let mut group = Group::from(self.start, size);
        while group.value < self.start {
            group.next();
        }
        while group.value <= self.end {
            invalid_ids.insert(group.value);
            group.next();
        }
    }
}

impl From<&str> for Range {
    fn from(pair: &str) -> Self {
        let (n1, n2) = pair.split_once("-").expect("pair is delimited by '-'");
        Self {
            start: n1.parse().expect("first pair item is a valid number"),
            end: n2.parse().expect("second pair item is a valid number"),
        }
    }
}

fn parse(data: &str) -> Vec<Range> {
    data.split(',').map(Range::from).collect()
}

fn main() {
    println!(
        "{}",
        parse(
            std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
                .expect("file exists")
                .as_str()
                .trim_end(),
        )
        .into_iter()
        .map(|range| range.sum_invalid_ids())
        .sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_group_988() {
        {
            let mut group = Group::from(998, 2);
            assert_eq!(group.value, 1010);
            group.next();
            assert_eq!(group.value, 1111);
            group.next();
            assert_eq!(group.value, 1212);
        }

        {
            let mut group = Group::from(998, 3);
            assert_eq!(group.value, 999);
            group.next();
            assert_eq!(group.value, 101010);
        }
        {
            let mut group = Group::from(998, 4);
            assert_eq!(group.value, 1111);
            group.next();
            assert_eq!(group.value, 2222);
        }
    }

    #[test]
    fn test_group_200000() {
        {
            let mut group = Group::from(200_000, 2);
            assert_eq!(group.value, 200_200);
            group.next();
            assert_eq!(group.value, 201_201);
        }

        {
            let mut group = Group::from(200_000, 3);
            assert_eq!(group.value, 202_020);
            group.next();
            assert_eq!(group.value, 212_121);
        }

        {
            let mut group = Group::from(200_000, 4);
            assert_eq!(group.value, 10_101_010);
            group.next();
            assert_eq!(group.value, 11_111_111);
        }

        {
            let mut group = Group::from(200_000, 5);
            assert_eq!(group.value, 1_010_101_010);
            group.next();
            assert_eq!(group.value, 1_111_111_111);
        }

        {
            let mut group = Group::from(200_000, 6);
            assert_eq!(group.value, 222_222);
            group.next();
            assert_eq!(group.value, 333_333);
        }
    }

    #[test]
    fn test_group_95() {
        assert_eq!(Group::from(95, 2).value, 99);
        assert_eq!(Group::from(95, 3).value, 111);
    }

    #[test]
    fn test_weird() {
        let mut group = Group::from(999_999, 2);
        assert_eq!(group.value, 999_999);
        group.next();
        assert_eq!(group.value, 10_001_000);
    }

    #[test]
    fn test_range() {
        assert_eq!(Range { start: 11, end: 22 }.sum_invalid_ids(), 11 + 22);
        assert_eq!(
            Range {
                start: 95,
                end: 115
            }
            .sum_invalid_ids(),
            99 + 111
        );
        assert_eq!(
            Range {
                start: 998,
                end: 1012
            }
            .sum_invalid_ids(),
            999 + 1010
        );
        assert_eq!(
            Range {
                start: 1188511880,
                end: 1188511890
            }
            .sum_invalid_ids(),
            1188511885
        );
        assert_eq!(
            Range {
                start: 2121212118,
                end: 2121212124
            }
            .sum_invalid_ids(),
            2121212121
        );
    }
}
