use ::std::collections::HashMap;
use ::std::collections::HashSet;

use std::sync::mpsc as mpsc;
use std::thread as thread;

mod input;

fn split_and_convert(instruction: &str) -> (&str, i16) {
    let (direction, rest) = instruction.split_at(1);
    let duration = rest.parse().unwrap();
    (direction, duration)
}

fn to_line_coordinates(coord_str: &str) -> HashMap<(i16, i16), i64> {
    let mut directions = HashMap::new();
    directions.insert("U", (0, 1));
    directions.insert("R", (1, 0));
    directions.insert("D", (0, -1));
    directions.insert("L", (-1, 0));

    let (mut x, mut y, mut dist) = (0, 0, 0);
    let mut coords = HashMap::new();
    for coord in coord_str.split(",") {
        let (direction, duration) = split_and_convert(coord);
        let (dx, dy) = directions[direction];

        // don't record corners
        x += dx;
        y += dy;
        dist += 1;
        for _ in 1..duration {
            if !coords.contains_key(&(x, y)) {
                coords.insert((x, y), dist);
            }
            x += dx;
            y += dy;
            dist += 1;
        }
    }
    coords
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || tx.send(to_line_coordinates(input::WIRE_1)));
    thread::spawn(move || tx2.send(to_line_coordinates(input::WIRE_2)));

    let wire_1 = rx.recv().unwrap();
    let wire_2 = rx.recv().unwrap();

    let wire_coords_1: HashSet<(i16, i16)> = wire_1.keys().cloned().collect();
    let wire_coords_2: HashSet<(i16, i16)> = wire_2.keys().cloned().collect();

    let crossings = wire_coords_1.intersection(&wire_coords_2);
    // println!("{:?}", crossings);
    let mut distances: Vec<i16> = crossings.clone().map(|(x, y)| x.abs() + y.abs()).collect();
    // println!("{:?}", distances);
    distances.sort();
    println!("Manhattan distances sorted {:?}", distances);

    let mut wire_distances: Vec<i64> = crossings
        .map(|coord| wire_1[coord] + wire_2[coord])
        .collect();
    // println!("{:?}", wire_distances);
    wire_distances.sort();
    println!("Wire distances sorted {:?}", wire_distances);
}
