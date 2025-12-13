type Number = u64;
#[derive(Debug)]
struct Problem {
    numbers: Vec<Number>,
    operation: fn(Problem) -> Number,
}

impl Problem {
    fn new(number: Number) -> Self {
        Self {
            numbers: vec![number],
            // Abort when not set
            operation: Problem::fail,
        }
    }

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

#[derive(Debug, Default)]
struct Problems(Vec<Problem>);

impl From<&str> for Problems {
    fn from(data: &str) -> Self {
        let mut out = Self::default();
        let mut lines = data.lines().peekable();
        lines
            .next()
            .expect("> 1 line")
            .split_whitespace()
            .for_each(|number| {
                out.0
                    .push(Problem::new(number.parse().expect("valid number")));
            });

        while let Some(line) = lines.next() {
            if lines.peek().is_none() {
                // Last line is operation
                for (ind, op) in line.split_whitespace().enumerate() {
                    out.0[ind].operation = match op {
                        "*" => Problem::multiplty,
                        "+" => Problem::add,
                        _ => panic!("invalid operation: {op}"),
                    }
                }
            } else {
                // Other lines are numbers
                for (ind, number) in line.split_whitespace().enumerate() {
                    out.0[ind]
                        .numbers
                        .push(number.parse().expect("valid number"));
                }
            }
        }

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
            .map(|problem| (problem.operation)(problem))
            .sum::<Number>()
    );
}
