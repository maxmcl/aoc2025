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
    fn recurse(
        &'a self,
        curr: &'a str,
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
        mut has_seen_dac: bool,
        mut has_seen_fft: bool,
    ) -> usize {
        const DAC: &str = "dac";
        const FFT: &str = "fft";
        const OUT: &str = "out";
        let key = (curr, has_seen_dac, has_seen_fft);
        if curr == OUT {
            return usize::from(has_seen_fft && has_seen_dac);
        } else if curr == DAC {
            has_seen_dac = true;
        } else if curr == FFT {
            has_seen_fft = true;
        }

        if let Some(count) = cache.get(&key) {
            return *count;
        }
        let Some(outputs) = self.0.get(curr) else {
            return 0;
        };
        let total = outputs
            .iter()
            .map(|output| self.recurse(output, cache, has_seen_dac, has_seen_fft))
            .sum();
        cache.insert(key, total);
        total
    }
    fn find_n_paths(&self) -> usize {
        const SVR: &str = "svr";
        let mut cache = HashMap::default();
        self.recurse(SVR, &mut cache, false, false)
    }
}

fn main() {
    let data =
        std::fs::read_to_string(std::env::args().nth(1).expect("filename")).expect("file exists");
    let graph = Graph::from(data.as_str());
    println!("{}", graph.find_n_paths());
}
