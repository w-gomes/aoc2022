use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::sorted;

fn main() -> anyhow::Result<()> {
    let file = File::open("inputs/day1.txt")?;
    let file = BufReader::new(file);
    let mut vec = Vec::new();
    let mut acc = 0;
    
    for line in file.lines() {
        if let Ok(num) = line {
            match num.parse::<u32>() {
                Ok(num) => acc += num,
                Err(_) => {
                    vec.push(acc);
                    acc = 0;
                }
            }
        }
    }
    println!("part1: {}", vec.iter().max().expect("huh?"));
    println!("part2: {}", sorted(vec).rev().take(3).sum::<u32>());

    Ok(())
}
