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
    fn contains(&self, ingredient: Id) -> bool {
        (self.start..=self.end).contains(&ingredient)
    }
}

#[derive(Debug)]
struct Database {
    ranges: Vec<Range>,
    ingredients: Vec<Id>,
}

impl From<&str> for Database {
    fn from(data: &str) -> Self {
        let (ranges, ingredients) = data
            .split_once("\n\n")
            .expect("ranges and ingredients split by \n\n");
        Self {
            ranges: ranges.lines().map(Range::from).collect(),
            ingredients: ingredients
                .lines()
                .map(|l| l.parse().expect("ingredient is a valid ID"))
                .collect(),
        }
    }
}

fn main() {
    let Database {
        ranges,
        ingredients,
    } = Database::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );
    println!(
        "{}",
        ingredients
            .into_iter()
            .filter(|ingredient| ranges.iter().any(|range| range.contains(*ingredient)))
            .count()
    );
}
