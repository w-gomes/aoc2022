#![feature(test)]
extern crate test;

use itertools::sorted;

fn part1(file: &str) -> u32 {
    let mut vec = Vec::new();
    let mut acc = 0;
    for line in file.lines() {
        match line.parse::<u32>() {
            Ok(num) => acc += num,
            Err(_) => {
                vec.push(acc);
                acc = 0;
            }
        }
    }
    *vec.iter().max().unwrap()
}

fn part2(file: &str) -> u32 {
    let mut vec = Vec::new();
    let mut acc = 0;
    for line in file.lines() {
        match line.parse::<u32>() {
            Ok(num) => acc += num,
            Err(_) => {
                vec.push(acc);
                acc = 0;
            }
        }
    }
    sorted(vec).rev().take(3).sum::<u32>()
}

fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string("inputs/day1.txt")?;

    println!("part1: {}", part1(&file));
    println!("part2: {}", part2(&file));

    Ok(())
}

use test::{black_box, Bencher};

#[bench]
fn run_part1(b: &mut Bencher) -> anyhow::Result<()> {
    let file = std::fs::read_to_string("inputs/day1.txt")?;
    b.iter(|| part1(black_box(&file)));
    Ok(())
}

#[bench]
fn run_part2(b: &mut Bencher) -> anyhow::Result<()> {
    let file = std::fs::read_to_string("inputs/day1.txt")?;
    b.iter(|| part2(black_box(&file)));
    Ok(())
}
