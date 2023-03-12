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

    let org: Vec<usize> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|e| e.parse::<usize>().unwrap())
        .collect();

    println!("{}", calc(&org, 12, 2));

    for noun in 0..=99 {
        for verb in 0..=99 {
            let c = calc(&org, noun, verb);
            if c == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}

fn calc(org: &[usize], noun: usize, verb: usize) -> usize {
    let mut nrs: Vec<usize> = org.to_vec();
    nrs[1] = noun;
    nrs[2] = verb;
    let mut index = 0;
    loop {
        let op = nrs[index];
        match op {
            1 => {
                let r = nrs[index + 3];
                nrs[r] = nrs[nrs[index + 1]] + nrs[nrs[index + 2]];
            }
            2 => {
                let r = nrs[index + 3];
                nrs[r] = nrs[nrs[index + 1]] * nrs[nrs[index + 2]];
            }
            99 => return nrs[0],
            _ => panic!("unknown op"),
        };

        index += 4;
    }
}
