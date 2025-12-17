type Pos = isize;
#[derive(Debug)]
struct Coord {
    x: Pos,
    y: Pos,
    z: Pos,
}

impl Coord {
    fn squared_distance(&self, other: &Coord) -> Pos {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CircuitId(usize);

#[derive(Debug, Default)]
struct JunctionBoxes(Vec<Coord>);

impl From<&str> for JunctionBoxes {
    fn from(data: &str) -> Self {
        JunctionBoxes(
            data.lines()
                .map(|line| {
                    let mut splits = line.split(',').map(|num| {
                        num.parse::<Pos>()
                            .expect("junction box coordinate is a valid number")
                    });
                    Coord {
                        x: splits.next().expect("x coordinate"),
                        y: splits.next().expect("y coordinate"),
                        z: splits.next().expect("z coordinate"),
                    }
                })
                .collect(),
        )
    }
}

fn main() {
    let n_connections = std::env::args()
        .nth(2)
        .expect("n connections value")
        .parse::<usize>()
        .expect("valid n connections value");
    let JunctionBoxes(boxes) = JunctionBoxes::from(
        std::fs::read_to_string(std::env::args().nth(1).expect("filename"))
            .expect("file exists")
            .as_str(),
    );

    let n = boxes.len();
    let mut id_pairs = Vec::with_capacity((1..n).sum());
    let mut circuit_to_boxes = Vec::with_capacity(n);
    let mut box_to_circuits = Vec::with_capacity(n);
    circuit_to_boxes.push(vec![0]);
    box_to_circuits.push(CircuitId(0));
    for i1 in 0..n - 1 {
        for i2 in i1 + 1..n {
            id_pairs.push((i1, i2, boxes[i1].squared_distance(&boxes[i2])));
            circuit_to_boxes.push(vec![i2]);
            box_to_circuits.push(CircuitId(i2));
        }
    }
    id_pairs.sort_unstable_by_key(|(_, _, distance)| *distance);
    id_pairs.truncate(n_connections);

    for (box_id1, box_id2, _) in id_pairs {
        let (circuit_id1, circuit_id2) = (box_to_circuits[box_id1], box_to_circuits[box_id2]);
        if circuit_id1 == circuit_id2 {
            continue;
        }

        // Merge the circuits
        let (ids, circuit_id) = match circuit_to_boxes[circuit_id1.0]
            .len()
            .cmp(&circuit_to_boxes[circuit_id2.0].len())
        {
            std::cmp::Ordering::Less => {
                // merge circuit 1 into circuit 2
                let mut temp = vec![];
                std::mem::swap(&mut temp, &mut circuit_to_boxes[circuit_id1.0]);
                (temp, circuit_id2)
            }
            _ => {
                // merge circuit 2 into circuit 1
                let mut temp = vec![];
                std::mem::swap(&mut temp, &mut circuit_to_boxes[circuit_id2.0]);
                (temp, circuit_id1)
            }
        };

        ids.iter().for_each(|id| box_to_circuits[*id] = circuit_id);
        circuit_to_boxes[circuit_id.0].extend(ids);
    }

    let mut circuit_sizes = circuit_to_boxes
        .into_iter()
        .map(|ids| ids.len())
        .collect::<Vec<_>>();

    circuit_sizes.sort_unstable();
    println!(
        "{}",
        circuit_sizes.into_iter().rev().take(3).product::<usize>()
    );
}
