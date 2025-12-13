type Id = u64;
#[derive(Debug)]
struct Range {
    start: Id,
    end: Id,
}

impl From<&str> for Range {
    fn from(data: &str) -> Self {
        let (start, end) = data
            .split_once('-')
            .expect("- separating start and end of range");
        Self {
            start: start.parse().expect("start is a valid number"),
            end: end.parse().expect("end is a valid number"),
        }
    }
}

impl Range {
    fn n_ingredients(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn merge(&mut self, other: Range) {
        self.start = std::cmp::min(self.start, other.start);
        self.end = std::cmp::max(self.end, other.end);
    }

    fn has_overlap(&self, other: &Range) -> bool {
        !(other.end < self.start || other.start > self.end)
    }
}

#[derive(Debug)]
struct Database(Vec<Range>);

impl From<&str> for Database {
    fn from(data: &str) -> Self {
        let (ranges, _) = data
            .split_once("\n\n")
            .expect("ranges and ingredients split by \n\n");
        Self(ranges.lines().map(Range::from).collect())
    }
}

impl Database {
    fn merge(mut self) -> Self {
        self.0.sort_unstable_by_key(|range| range.start);
        let mut new = Vec::with_capacity(self.0.len());
        let mut iter = self.0.into_iter();
        new.push(iter.next().expect("at least 1 range"));
        let mut n = 0;
        for next in iter {
            if new[n].has_overlap(&next) {
                new[n].merge(next);
            } else {
                new.push(next);
                n += 1;
            }
        }
        Self(new)
    }
}

fn main() {
    let database = Database::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    )
    .merge();
    println!(
        "{}",
        database
            .0
            .into_iter()
            .map(|range| range.n_ingredients())
            .sum::<usize>()
    );
}
