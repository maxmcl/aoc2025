use std::collections::{HashMap, HashSet};

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

type BoxId = usize;

fn merge(
    circuits_map: &mut HashMap<CircuitId, HashSet<BoxId>>,
    boxes_map: &mut HashMap<BoxId, CircuitId>,
    circuit_id1: CircuitId,
    circuit_id2: CircuitId,
    box_id2: BoxId,
) -> CircuitId {
    if circuit_id1 != circuit_id2 {
        // Merge the 2nd circuit in the 1st
        // TODO: merge smallest in largest
        let mut merged_ids = unsafe { circuits_map.remove(&circuit_id2).unwrap_unchecked() };
        // box_id2 is temporarily not in boxes_map, skip it
        merged_ids.remove(&box_id2);
        // replace ids pointing to circuit_id2 by circuit_id1
        merged_ids.iter().for_each(|id| {
            *unsafe { boxes_map.get_mut(id).unwrap_unchecked() } = circuit_id1;
        });
        unsafe { circuits_map.get_mut(&circuit_id1).unwrap_unchecked() }
            .extend(merged_ids.into_iter().chain(std::iter::once(box_id2)));
    }
    circuit_id1
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
    for i1 in 0..n - 1 {
        for i2 in i1 + 1..n {
            id_pairs.push((i1, i2, boxes[i1].squared_distance(&boxes[i2])));
        }
    }
    id_pairs.sort_unstable_by_key(|(_, _, distance)| *distance);
    id_pairs.truncate(n_connections);

    // TODO: Vec<HashSet> and Vec<CircuitId> instead or Vec<Vec> (matrix)
    let mut circuits_map = HashMap::<CircuitId, HashSet<BoxId>>::default();
    let mut boxes_map = HashMap::<BoxId, CircuitId>::default();
    let mut curr_circuit_id = CircuitId(0);

    for (box_id1, box_id2, _) in id_pairs {
        println!("{box_id1} & {box_id2}");
        let circuit_id = match (boxes_map.remove(&box_id1), boxes_map.remove(&box_id2)) {
            (Some(circuit_id1), Some(circuit_id2)) => merge(
                &mut circuits_map,
                &mut boxes_map,
                circuit_id1,
                circuit_id2,
                box_id2,
            ),
            (Some(circuit_id), None) => {
                // Insert box ID 2 in box ID 1's circuit
                unsafe { circuits_map.get_mut(&circuit_id).unwrap_unchecked() }.insert(box_id2);
                circuit_id
            }
            (None, Some(circuit_id)) => {
                // Insert box ID 1 in box ID 2's circuit
                unsafe { circuits_map.get_mut(&circuit_id).unwrap_unchecked() }.insert(box_id1);
                circuit_id
            }
            (None, None) => {
                // Both boxes don't belong to any circuit, create one
                let id = curr_circuit_id;
                let ids = circuits_map.entry(id).or_default();
                curr_circuit_id.0 += 1;
                ids.insert(box_id1);
                ids.insert(box_id2);
                id
            }
        };
        boxes_map.insert(box_id1, circuit_id);
        boxes_map.insert(box_id2, circuit_id);
    }

    let mut circuit_sizes = circuits_map
        .into_values()
        .map(|ids| ids.len())
        .collect::<Vec<_>>();

    circuit_sizes.sort_unstable();
    println!(
        "{}",
        circuit_sizes.into_iter().rev().take(3).product::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_merge() {
        let mut circuits_map = HashMap::<CircuitId, HashSet<BoxId>>::default();
        let circuit_id1 = CircuitId(0);
        {
            let ids = circuits_map.entry(circuit_id1).or_default();
            ids.insert(0);
            ids.insert(1);
        }
        let circuit_id2 = CircuitId(2);
        {
            let ids = circuits_map.entry(circuit_id2).or_default();
            ids.insert(2);
            ids.insert(3);
            ids.insert(4);
        }
        let mut boxes_map = HashMap::<BoxId, CircuitId>::default();
        boxes_map.insert(0, circuit_id1);
        boxes_map.insert(1, circuit_id1);
        boxes_map.insert(2, circuit_id2);
        boxes_map.insert(3, circuit_id2);
        boxes_map.insert(4, circuit_id2);
        merge(
            &mut circuits_map,
            &mut boxes_map,
            circuit_id1,
            circuit_id2,
            2,
        );

        let ids = circuits_map.get(&circuit_id1);
        assert!(ids.is_some());
        assert!(ids.unwrap().len() == 5);
        assert!(circuits_map.get(&circuit_id2).is_none());
        boxes_map.get_mut(&2).map(|id| *id = circuit_id1);
        assert!(boxes_map.values().all(|id| *id == circuit_id1));
    }
}
