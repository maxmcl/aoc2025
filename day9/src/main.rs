#![feature(iter_map_windows)]

type Pos = isize;
#[derive(Debug, Clone, Copy)]
struct Coord {
    x: Pos,
    y: Pos,
}

#[derive(Debug, Clone, Copy)]
struct BoundingBox {
    min: Coord,
    max: Coord,
}

impl Coord {
    fn area(&self, other: &Coord) -> Pos {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }

    fn get_bounding_box(&self, other: &Coord) -> BoundingBox {
        let mut xs = [self.x, other.x];
        xs.sort_unstable();
        let mut ys = [self.y, other.y];
        ys.sort_unstable();
        BoundingBox {
            min: Coord { x: xs[0], y: ys[0] },
            max: Coord { x: xs[1], y: ys[1] },
        }
    }
}

impl BoundingBox {
    fn intersects(&self, other: &Self) -> bool {
        /*
           #--#
           |  |  #--#
           #--#  |  |
                 #--#
         self.max.y <= other.min.y => no overlap
             #--#
             |  |
             #--#
        self.max.y > other.min.y => overlap
         */
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
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

fn is_contained_in(within: &[BoundingBox], search: &BoundingBox) -> bool {
    !within.iter().any(|bbox| search.intersects(bbox))
}

fn build_bounding_boxes_from_vertices(vertices: &[Coord]) -> Vec<BoundingBox> {
    vertices
        .iter()
        .chain(std::iter::once(&vertices[0]))
        .map_windows(|[start, end]| start.get_bounding_box(end))
        .collect()
}

fn main() {
    let Tiles(tiles) = Tiles::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );

    let n = tiles.len();
    let pairs_and_area = {
        let mut out = Vec::with_capacity(1);
        for i1 in 0..n {
            let tile1 = &tiles[i1];
            // Skip 'adjacent' tiles
            for tile2 in tiles.iter().take(n - 1).skip(i1 + 2) {
                out.push((tile1, tile2, tile1.area(tile2)));
            }
        }
        // Attempt largest rectangles first
        out.sort_unstable_by_key(|(_, _, area)| -area);
        out
    };
    let bboxes = build_bounding_boxes_from_vertices(&tiles);
    println!(
        "{:?}",
        pairs_and_area
            .into_iter()
            .find(|(tile1, tile2, _)| is_contained_in(&bboxes, &tile1.get_bounding_box(tile2)))
            .expect("> 1 pair")
    )
}
