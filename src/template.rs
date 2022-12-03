#![feature(test)]
extern crate test;

use std::io::{BufRead, BufReader};

static FILE: &str = include_str!("day?.txt");

fn main() -> anyhow::Result<()> {
    let file = std::fs::File::open("inputs/day?.txt")?;
    let file = BufReader::new(file);

    Ok(())
}

use test::{black_box, Bencher};

#[bench]
fn run_part1(b: &mut Bencher) -> anyhow::Result<()> {
    b.iter(|| part1(black_box()));
    Ok(())
}

#[bench]
fn run_part2(b: &mut Bencher) -> anyhow::Result<()> {
    b.iter(|| part2(black_box()));
    Ok(())
}
