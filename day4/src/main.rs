use std::collections::HashSet;

type Pos = isize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: Pos,
    y: Pos,
}

impl Coord {
    fn get_neighbors(&self) -> impl Iterator<Item = Coord> {
        const SHIFTS: [Coord; 8] = [
            Coord { x: -1, y: 0 },
            Coord { x: -1, y: 1 },
            Coord { x: 0, y: 1 },
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: -1 },
            Coord { x: 0, y: -1 },
            Coord { x: -1, y: -1 },
        ];

        SHIFTS.into_iter().map(|shift| Coord {
            x: self.x + shift.x,
            y: self.y + shift.y,
        })
    }
}

#[derive(Debug)]
struct Diagram {
    rolls: HashSet<Coord>,
    n_rows: Pos,
    n_cols: Pos,
}

impl From<&str> for Diagram {
    fn from(data: &str) -> Self {
        let mut coord = Coord { x: 0, y: 0 };
        let mut rolls = HashSet::default();
        for c in data.chars() {
            match c {
                '.' => (),
                '@' => {
                    rolls.insert(coord);
                }
                '\n' => {
                    coord.x += 1;
                    coord.y = 0;
                    continue;
                }
                _ => panic!("invalid char in diagram: {c}"),
            }
            coord.y += 1;
        }

        let n_cols = rolls
            .iter()
            .map(|coord| coord.y)
            .max()
            .expect(">= 1 element in grid")
            + 1;
        Self {
            rolls,
            n_rows: coord.x,
            n_cols,
        }
    }
}

impl Diagram {
    fn is_in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.x < self.n_rows && coord.y >= 0 && coord.y < self.n_cols
    }
    fn is_accessible(&self, coord: &Coord) -> bool {
        const MIN_ROLLS: usize = 4;
        coord
            .get_neighbors()
            .filter(|neighbor| self.is_in_bounds(&neighbor) && self.rolls.contains(&neighbor))
            .count()
            < MIN_ROLLS
    }
}

fn main() {
    let diagram = Diagram::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );
    dbg!(&diagram);
    println!(
        "{}",
        diagram
            .rolls
            .iter()
            .filter(|roll| diagram.is_accessible(roll))
            .count()
    );
}

#[cfg(test)]
mod tests {}
