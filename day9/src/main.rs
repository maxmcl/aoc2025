type Pos = isize;
#[derive(Debug)]
struct Coord {
    x: Pos,
    y: Pos,
}

impl Coord {
    fn area(&self, other: &Coord) -> Pos {
        ((self.x - other.x + 1) * (self.y - other.y + 1)).abs()
    }
}

#[derive(Debug, Default)]
struct Tiles(Vec<Coord>);

impl From<&str> for Tiles {
    fn from(data: &str) -> Self {
        Tiles(
            data.lines()
                .map(|line| {
                    let mut splits = line.split(',').map(|num| {
                        num.parse::<Pos>()
                            .expect("tile coordinate is a valid number")
                    });
                    Coord {
                        x: splits.next().expect("x coordinate"),
                        y: splits.next().expect("y coordinate"),
                    }
                })
                .collect(),
        )
    }
}

fn main() {
    let Tiles(tiles) = Tiles::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );

    let n = tiles.len();
    let mut max = 0;
    for i1 in 0..n - 1 {
        let tile1 = &tiles[i1];
        for tile2 in tiles.iter().take(n).skip(i1 + 1) {
            max = std::cmp::max(max, tile1.area(tile2));
        }
    }
    println!("{max}");
}
