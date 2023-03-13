// #![allow(dead_code)]
// #![allow(unused_variables)]

use std::{
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

#[derive(PartialEq)]
enum LineType {
    Horizontal,
    Vertical,
}

type Coord = (i32, i32);
type Line = (LineType, i32, i32, i32);

fn main() {
    let lines = lines_from_file("in");
    let segs1 = segments(lines.first().unwrap());
    let segs2 = segments(lines.last().unwrap());

    let mut inters: Vec<Coord> = Vec::new();
    inters = all_intersections(inters, &segs1, &segs2);
    inters = all_intersections(inters, &segs2, &segs1);

    println!("{}", min_dist(&inters));
}

fn min_dist(inters: &[Coord]) -> i32 {
    inters.iter().map(manhattan_distance).filter(|i| *i > 0).min().unwrap()
}

fn all_intersections(mut inters: Vec<Coord>, segs1: &[Line], segs2: &[Line]) -> Vec<Coord> {
    for h in segs1.iter().filter(|s| s.0 == LineType::Horizontal) {
        for v in segs2.iter().filter(|s| s.0 == LineType::Vertical) {
            if let Some(c) = intersection(h, v) {
                inters.push(c);
            }
        }
    }

    inters
}

fn manhattan_distance((x, y): &Coord) -> i32 {
    x.abs() + y.abs()
}

fn segments(line: &str) -> Vec<Line> {
    let instructions: Vec<&str> = line.split(',').collect();
    let mut result: Vec<Line> = Vec::new();
    let mut pos: Coord = (0, 0);
    for ins in instructions {
        let dir = ins.chars().next().unwrap();
        let nr = ins[1..].parse::<i32>().unwrap();
        pos = match dir {
            'U' => {
                let nxt = (pos.0, pos.1 + nr);
                result.push((LineType::Vertical, pos.0, pos.1, nxt.1));
                nxt
            }
            'R' => {
                let nxt = (pos.0 + nr, pos.1);
                result.push((LineType::Horizontal, pos.1, pos.0, nxt.0));
                nxt
            }
            'D' => {
                let nxt = (pos.0, pos.1 - nr);
                result.push((LineType::Vertical, pos.0, nxt.1, pos.1));
                nxt
            }
            'L' => {
                let nxt = (pos.0 - nr, pos.1);
                result.push((LineType::Horizontal, pos.1, nxt.0, pos.0));
                nxt
            }
            _ => panic!(),
        };
    }

    result
}

fn intersection(horizontal: &Line, vertical: &Line) -> Option<Coord> {
    if (horizontal.2..=horizontal.3).contains(&vertical.1)
        && (vertical.2..=vertical.3).contains(&horizontal.1)
    {
        Some((vertical.1, horizontal.1))
    } else {
        None
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(
            intersection(
                &(LineType::Horizontal, 0, 0, 5),
                &(LineType::Vertical, 2, -2, 2)
            ),
            Some((2, 0))
        );
        assert_eq!(
            intersection(
                &(LineType::Horizontal, 0, 0, 5),
                &(LineType::Vertical, 1, -1, 1)
            ),
            Some((1, 0))
        );
        assert_eq!(
            intersection(
                &(LineType::Horizontal, 0, 0, 5),
                &(LineType::Vertical, 0, -1, 1)
            ),
            Some((0, 0))
        );
        assert_eq!(
            intersection(
                &(LineType::Horizontal, 0, 0, 5),
                &(LineType::Vertical, 5, -1, 1)
            ),
            Some((5, 0))
        );
        assert_eq!(
            intersection(
                &(LineType::Horizontal, 0, 0, 5),
                &(LineType::Vertical, 2, 6, 8)
            ),
            None
        );
        assert_eq!(
            intersection(
                &(LineType::Horizontal, 0, 0, 5),
                &(LineType::Vertical, -2, -2, 2)
            ),
            None
        );
    }
}
