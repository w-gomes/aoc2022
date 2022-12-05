#[macro_use]
extern crate maplit;

use itertools::sorted;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::HashMap;

static FILE: &str = include_str!("day5.txt");

struct Movement {
    quantity: u16,
    src: u16,
    dst: u16,
}

fn move_proc(input: &str) -> IResult<&str, u16> {
    preceded(tag("move "), map_res(digit1, str::parse))(input)
}

fn from_proc(input: &str) -> IResult<&str, u16> {
    preceded(tag(" from "), map_res(digit1, str::parse))(input)
}

fn to_proc(input: &str) -> IResult<&str, u16> {
    preceded(tag(" to "), map_res(digit1, str::parse))(input)
}

fn parser(input: &str) -> IResult<&str, Movement> {
    let (input, (quantity, src, dst)) = tuple((move_proc, from_proc, to_proc))(input)?;
    Ok((input, Movement { quantity, src, dst }))
}

fn parse(file: &str) -> Vec<Movement> {
    file.lines()
        .skip(10)
        .map(|line| {
            let (_, mov) = parser(line).unwrap();
            mov
        })
        .collect::<Vec<Movement>>()
}

fn part1(movs: &Vec<Movement>, mut stack: HashMap<u16, Vec<char>>) {
    movs.iter().for_each(|mov| {
        for _ in 1..=mov.quantity {
            let src_list = stack.get_mut(&mov.src).unwrap();
            let item = src_list.pop().unwrap();
            let dst_list = stack.get_mut(&mov.dst).unwrap();
            dst_list.push(item);
        }
    });
    sorted(stack.iter()).for_each(|stack| print!("{}", stack.1.last().unwrap()));
    print!("{}", "\n");
}

fn part2(movs: &Vec<Movement>, mut stack: HashMap<u16, Vec<char>>) {
    movs.iter().for_each(|mov| {
        let mut temp = Vec::new();
        for _ in 1..=mov.quantity {
            let src_list = stack.get_mut(&mov.src).unwrap();
            let item = src_list.pop().unwrap();
            temp.push(item);
        }
        let dst_list = stack.get_mut(&mov.dst).unwrap();
        if temp.len() > 1 {
            temp.reverse();
            dst_list.append(&mut temp);
        } else {
            dst_list.push(temp[0]);
        }
    });
    sorted(stack.iter()).for_each(|stack| print!("{}", stack.1.last().unwrap()));
    print!("{}", "\n");
}

fn main() {
    let stack = hashmap! {
        1 => vec!['H', 'C', 'R'],
        2 => vec!['B', 'J', 'H', 'L', 'S', 'F'],
        3 => vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],
        4 => vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],
        5 => vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'],
        6 => vec!['T', 'H', 'C', 'G'],
        7 => vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'],
        8 => vec!['R', 'J', 'Q', 'G', 'C'],
        9 => vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S'],
    };
    let movements = parse(FILE);
    part1(&movements, stack.clone());
    part2(&movements, stack.clone());
}
