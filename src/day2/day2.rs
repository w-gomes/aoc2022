#![feature(test)]
extern crate test;

static FILE: &str = include_str!("day2.txt");

fn part1(f: &str) -> u32 {
    f.lines()
        .map(
            |line| match (line.get(0..1).unwrap(), line.get(2..).unwrap()) {
                ("A", "Y") => 8,
                ("A", "X") => 4,
                ("A", "Z") => 3,
                ("B", "Y") => 5,
                ("B", "X") => 1,
                ("B", "Z") => 9,
                ("C", "Y") => 2,
                ("C", "X") => 7,
                ("C", "Z") => 6,
                (_, _) => unreachable!(),
            },
        )
        .sum()
}

fn part2(f: &str) -> u32 {
    f.lines()
        .map(
            |line| match (line.get(0..1).unwrap(), line.get(2..).unwrap()) {
                ("A", "Y") => 4,
                ("A", "X") => 3,
                ("A", "Z") => 8,
                ("B", "Y") => 5,
                ("B", "X") => 1,
                ("B", "Z") => 9,
                ("C", "Y") => 6,
                ("C", "X") => 2,
                ("C", "Z") => 7,
                (_, _) => unreachable!(),
            },
        )
        .sum()
}

fn main() -> anyhow::Result<()> {
    println!("part1 {}", part1(FILE));
    println!("part2 {}", part2(FILE));
    Ok(())
}

use test::{black_box, Bencher};

#[bench]
fn run_part1(b: &mut Bencher) -> anyhow::Result<()> {
    b.iter(|| part1(black_box(FILE)));
    Ok(())
}

#[bench]
fn run_part2(b: &mut Bencher) -> anyhow::Result<()> {
    b.iter(|| part2(black_box(FILE)));
    Ok(())
}
