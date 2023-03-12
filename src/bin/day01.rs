#![allow(dead_code)]
#![allow(unused_variables)]

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

fn main() {
    let lines = lines_from_file("in");

    let result1: u64 = lines
        .iter()
        .map(|e| fuel_for_weight(e.parse::<u64>().unwrap()))
        .sum();
    println!("{}", result1);

    let result2: u64 = lines
        .iter()
        .map(|e| fuel_for_module(e.parse::<u64>().unwrap(), 0))
        .sum();
    println!("{}", result2);
}

fn fuel_for_weight(weight: u64) -> u64 {
    let third = weight / 3;
    if third < 3 {
        return 0;
    }

    third - 2
}

fn fuel_for_module(weight: u64, total: u64) -> u64 {
    if weight == 0 {
        return total;
    }

    let f = fuel_for_weight(weight);
    fuel_for_module(f, total + f)
}
