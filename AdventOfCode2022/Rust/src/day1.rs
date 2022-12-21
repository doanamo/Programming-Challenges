use itertools::Itertools;

pub fn run()
{
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let supplies = input.split("\n\n")
        .map(|text| text.split_whitespace()
            .map(|line| line.parse::<u32>().unwrap()).sum())
        .sorted().collect::<Vec<u32>>();

    println!("Day 1-1 answer: {}", supplies.iter().last().unwrap());
    println!("Day 1-2 answer: {}", supplies.iter().rev().take(3).sum::<u32>());
}
