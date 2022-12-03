#![feature(test)]
#![feature(const_try)]
extern crate test;

use itertools::sorted;

static FILE: &str = include_str!("day1.txt");

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
    println!("part1: {}", part1(FILE));
    println!("part2: {}", part2(FILE));

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
