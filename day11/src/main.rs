use std::collections::HashMap;

#[derive(Debug, Default)]
struct Graph<'a>(HashMap<&'a str, Vec<&'a str>>);

impl<'a> From<&'a str> for Graph<'a> {
    fn from(data: &'a str) -> Self {
        Self(
            data.lines()
                .map(|line| {
                    let (input, outputs) = line
                        .split_once(": ")
                        .expect("input outputs delimiter present");
                    (input, outputs.split(' ').collect())
                })
                .collect(),
        )
    }
}

impl<'a> Graph<'a> {
    fn recurse(&'a self, curr: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
        const OUT: &str = "out";
        if curr == OUT {
            return 1;
        } else if let Some(count) = cache.get(curr) {
            return *count;
        }
        let Some(outputs) = self.0.get(curr) else {
            return 0;
        };
        let total = outputs
            .iter()
            .map(|output| self.recurse(output, cache))
            .sum();
        cache.insert(curr, total);
        total
    }
    fn find_n_paths(&self) -> usize {
        const YOU: &str = "you";
        let mut cache = HashMap::default();
        self.recurse(YOU, &mut cache)
    }
}

fn main() {
    let data =
        std::fs::read_to_string(std::env::args().nth(1).expect("filename")).expect("file exists");
    let graph = Graph::from(data.as_str());
    println!("{}", graph.find_n_paths());
}
