#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Debug)]
struct Half {
    pub value: u64,
    base: u64,
    n: u32, // an even number by construction
    next_base: u64,
}

impl Half {
    fn from_value(value: u64) -> Self {
        let mut n = Half::get_n_digits(value);
        let (base, next_base) = if n.is_multiple_of(2) {
            n /= 2;
            let next_base = 10_u64.pow(n);
            (value / next_base, next_base)
        } else {
            // Odd value, skip to next even digit number
            n = n.div_ceil(2);
            let next_base = (10u64).pow(n);
            (next_base / 10, next_base)
        };
        Self {
            value: Half::compute_value(base, n),
            base,
            n,
            next_base,
        }
    }

    fn next(&mut self) {
        self.base += 1;
        if self.base == self.next_base {
            // Skip odd numbers
            self.n += 1;
            self.next_base *= 10;
        }
        self.value = Half::compute_value(self.base, self.n);
    }

    fn compute_value(base: u64, n: u32) -> u64 {
        base * 10_u64.pow(n) + base
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
        /* an invalid ID is made of 2 repeated sequences of digits,
          therefore, it must have an even length. E.g.
          22, 6464, 123123

          therefore, since the number must be repeated, only the
          first half of the numbers in the range can be considered. E.g.

          83-113
          |   |> length 3, ignored
          |
          v length 2, first half = 8, must be >= 8
          candidates: 88, 99
        */
        let mut half = Half::from_value(self.start);
        while half.value < self.start {
            half.next();
        }
        let mut sum = 0;
        while half.value <= self.end {
            sum += half.value;
            half.next();
        }
        sum
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
