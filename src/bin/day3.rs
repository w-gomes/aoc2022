#![feature(iter_array_chunks)]
#[macro_use]
extern crate maplit;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string("inputs/day3.txt")?;
    let priority = hashmap! {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
    };

    let part1: u32 = file
        .lines()
        .map(|line| {
            let size = line.len();
            let (first, second) = line.split_at(size / 2);

            let first: HashSet<_> = first.chars().collect();
            let second: HashSet<_> = second.chars().collect();

            let mut common: HashSet<_> = first.intersection(&second).collect();
            let value = priority[common.drain().next().unwrap()];
            value
        })
        .sum();
    println!("part1: {}", part1);

    let part2: u32 = file
        .lines()
        .array_chunks::<3>()
        .map(|chunks| {
            let first: HashSet<_> = chunks[0].chars().collect();
            let second: HashSet<_> = chunks[1].chars().collect();
            let third: HashSet<_> = chunks[2].chars().collect();

            let bitand = &first & &second;
            let mut common: HashSet<_> = bitand.intersection(&third).collect();
            let value = priority[common.drain().next().unwrap()];
            value
        })
        .sum();
    println!("part2: {}", part2);

    Ok(())
}
