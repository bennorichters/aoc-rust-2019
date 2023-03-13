// #![allow(dead_code)]
// #![allow(unused_variables)]

use std::{
    cmp::min,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|e| e.expect("Could not parse line"))
        .collect()
}

type Coord = (i32, i32);
type Line = (i32, i32, i32);

fn main() {
    let lines = lines_from_file("in");
    let (hs1, vs1) = segments(lines.first().unwrap());
    let (hs2, vs2) = segments(lines.last().unwrap());

    let r1 = min_dist(&hs1, &vs2);
    let r2 = min_dist(&hs2, &vs1);
    let result = min(r1, r2);
    println!("{}", result);
}

fn min_dist(hs: &Vec<Line>, vs: &Vec<Line>) -> i32 {
    let mut result = 0;
    for h in hs {
        for v in vs {
            if let Some(c) = intersection(h, v) {
                let dist = manhattan_distance(c);
                if dist > 0 {
                    result = if result == 0 { dist } else { min(result, dist) };
                }
            }
        }
    }

    result
}

fn manhattan_distance((x, y): Coord) -> i32 {
    x.abs() + y.abs()
}

fn segments(line: &str) -> (Vec<Line>, Vec<Line>) {
    let instructions: Vec<&str> = line.split(',').collect();
    let mut hs: Vec<Line> = Vec::new();
    let mut vs: Vec<Line> = Vec::new();
    let mut pos: Coord = (0, 0);
    for ins in instructions {
        let dir = ins.chars().next().unwrap();
        let nr = ins[1..].parse::<i32>().unwrap();
        pos = match dir {
            'U' => {
                let nxt = (pos.0, pos.1 + nr);
                vs.push((pos.0, pos.1, nxt.1));
                nxt
            }
            'R' => {
                let nxt = (pos.0 + nr, pos.1);
                hs.push((pos.1, pos.0, nxt.0));
                nxt
            }
            'D' => {
                let nxt = (pos.0, pos.1 - nr);
                vs.push((pos.0, nxt.1, pos.1));
                nxt
            }
            'L' => {
                let nxt = (pos.0 - nr, pos.1);
                hs.push((pos.1, nxt.0, pos.0));
                nxt
            }
            _ => panic!(),
        };
    }

    (hs, vs)
}

fn intersection(horizontal: &Line, vertical: &Line) -> Option<Coord> {
    if (horizontal.1..=horizontal.2).contains(&vertical.0)
        && (vertical.1..=vertical.2).contains(&horizontal.0)
    {
        Some((vertical.0, horizontal.0))
    } else {
        None
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(&(0, 0, 5), &(2, -2, 2)), Some((2, 0)));
        assert_eq!(intersection(&(0, 0, 5), &(1, -1, 1)), Some((1, 0)));
        assert_eq!(intersection(&(0, 0, 5), &(0, -1, 1)), Some((0, 0)));
        assert_eq!(intersection(&(0, 0, 5), &(5, -1, 1)), Some((5, 0)));
        assert_eq!(intersection(&(0, 0, 5), &(2, 6, 8)), None);
        assert_eq!(intersection(&(0, 0, 5), &(-2, -2, 2)), None);
    }
}
