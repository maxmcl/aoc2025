use std::collections::HashSet;

type Pos = isize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: Pos,
    y: Pos,
}

impl Coord {
    fn split(&self) -> impl Iterator<Item = Coord> {
        const SHIFTS: [Coord; 2] = [Coord { x: 0, y: 1 }, Coord { x: 0, y: -1 }];
        SHIFTS.into_iter().map(|shift| Coord {
            x: self.x + shift.x,
            y: self.y + shift.y,
        })
    }
}

#[derive(Debug)]
struct Manifold {
    splitters: HashSet<Coord>,
    start: Coord,
    n_rows: Pos,
}

impl From<&str> for Manifold {
    fn from(data: &str) -> Self {
        let mut coord = Coord { x: 0, y: 0 };
        let mut splitters = HashSet::<Coord>::default();
        let mut start = None;
        for c in data.chars() {
            match c {
                '.' => (),
                'S' => {
                    start = Some(coord);
                }
                '^' => {
                    splitters.insert(coord);
                }
                '\n' => {
                    coord.x += 1;
                    coord.y = 0;
                    continue;
                }
                _ => panic!("unexpected character in Manifold: {c}"),
            }
            coord.y += 1;
        }

        Self {
            start: start.expect("start was found"),
            n_rows: splitters
                .iter()
                .map(|coord| coord.x)
                .max()
                .expect("> 1 splitter"),
            splitters,
        }
    }
}

impl Manifold {
    fn find_splitter_below(&self, coord: Coord) -> Option<Coord> {
        (coord.x + 1..=self.n_rows).find_map(|x| {
            let coord = Coord { x, y: coord.y };
            if self.splitters.contains(&coord) {
                Some(coord)
            } else {
                None
            }
        })
    }

    fn count_splits(&self) -> usize {
        let mut to_visit = vec![self.start];
        let mut visited = HashSet::<Coord>::default();
        let mut count = 0;
        while let Some(coord) = to_visit.pop() {
            if let Some(splitter) = self.find_splitter_below(coord) {
                if !visited.insert(splitter) {
                    continue;
                }
                count += 1;
                for split in splitter.split() {
                    to_visit.push(split);
                }
            }
        }

        count
    }
}

fn main() {
    println!(
        "{}",
        Manifold::from(
            std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
                .expect("file exists")
                .as_str()
        )
        .count_splits()
    );
}
