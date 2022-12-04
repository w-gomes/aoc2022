#![feature(test)]
extern crate test;

use std::collections::HashSet;

static FILE: &str = include_str!("day4.txt");

fn parse(file: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    file.lines()
        .map(|line| {
            let mut splitted: Vec<&str> = line.split(|mid| mid == ',').collect();
            let right = splitted.pop().unwrap();
            let left = splitted.pop().unwrap();
            let right_span: Vec<_> = right.split(|mid| mid == '-').collect();
            let left_span: Vec<_> = left.split(|mid| mid == '-').collect();
            let right_span: Vec<_> = right_span
                .into_iter()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            let left_span: Vec<_> = left_span
                .into_iter()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            (
                (left_span[0]..=left_span[1]).collect::<HashSet<_>>(),
                (right_span[0]..=right_span[1]).collect::<HashSet<_>>(),
            )
        })
        .collect()
}

fn part1(list: &Vec<(HashSet<u32>, HashSet<u32>)>) -> usize {
    list.iter()
        .filter(|tuple| tuple.0.is_superset(&tuple.1) || tuple.1.is_superset(&tuple.0))
        .collect::<Vec<_>>()
        .len()
}

fn part2(list: &Vec<(HashSet<u32>, HashSet<u32>)>) -> usize {
    list.iter()
        .filter(|tuple| {
            let bitand = &tuple.0 & &tuple.1;
            !bitand.is_empty()
        })
        .collect::<Vec<_>>()
        .len()
}

fn main() -> anyhow::Result<()> {
    let list = parse(FILE);
    println!("part1: {}", part1(&list));
    println!("part2: {}", part2(&list));

    Ok(())
}

use test::{black_box, Bencher};

#[bench]
fn run_part1(b: &mut Bencher) -> anyhow::Result<()> {
    let list = parse(FILE);
    b.iter(|| part1(black_box(list)));
    Ok(())
}

#[bench]
fn run_part2(b: &mut Bencher) -> anyhow::Result<()> {
    let list = parse(FILE);
    b.iter(|| part2(black_box(list)));
    Ok(())
}
