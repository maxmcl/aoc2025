const N_PRESENTS: usize = 6;
#[derive(Debug, Default)]
struct Problem {
    area: usize,
    present_counts_to_fit: [usize; N_PRESENTS],
}

#[derive(Debug, Default)]
struct Input {
    present_areas: [usize; N_PRESENTS],
    problems: Vec<Problem>,
}

impl From<&str> for Input {
    fn from(data: &str) -> Self {
        let mut groups = data.split("\n\n");
        let mut out = Self::default();
        for idx in 0..N_PRESENTS {
            let group = groups
                .next()
                .unwrap_or_else(|| panic!("group {idx} exists"));
            out.present_areas[idx] = group
                .strip_prefix(&format!("{idx}:\n"))
                .expect("group start")
                .chars()
                .filter(|&c| c == '#')
                .count();
        }
        for line in groups.next().expect("problems group exists").lines() {
            let (area, indices) = line.split_once(": ").expect("colon delimiter");
            let area = area.split_once('x').expect("area width x height");
            out.problems.push(Problem {
                area: area.0.parse::<usize>().expect("valid width")
                    * area.1.parse::<usize>().expect("valid height"),
                present_counts_to_fit: indices
                    .split(' ')
                    .map(|num| num.parse::<usize>().expect("valid count"))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!("{N_PRESENTS} indicies")),
            })
        }
        out
    }
}

impl Input {
    fn solve_single(&self, problem: &Problem) -> bool {
        /*
          3 scenarios:
          - the area of all the required shapes > the area of the tree (impossible)
          - the max area of all the required shapes <= the area of the tree (trivial)
          - otherwise, the shapes have to overlap, which is hard to solve but doesn't
          happen in the input which is dumb
        */
        const MAX_PRESENT_AREA: usize = 9;
        let max_area = MAX_PRESENT_AREA * problem.present_counts_to_fit.iter().sum::<usize>();
        if max_area <= problem.area {
            return true;
        }

        let required_area = problem
            .present_counts_to_fit
            .iter()
            .enumerate()
            .map(|(ind, count)| count * self.present_areas[ind])
            .sum::<usize>();
        if required_area > problem.area {
            return false;
        }

        panic!("hard case");
    }
    fn solve(&self) -> usize {
        self.problems
            .iter()
            .filter(|problem| self.solve_single(problem))
            .count()
    }
}

fn main() {
    let input = Input::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );
    println!("{}", input.solve());
}
