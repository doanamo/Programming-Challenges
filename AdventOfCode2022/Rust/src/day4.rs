use sscanf::sscanf;
use itertools::Itertools;

pub fn run()
{
    let input = include_str!("../input/day4.txt");
    let part2 = input.lines().map(|line| sscanf!(line, "{u32}-{u32},{u32}-{u32}").unwrap());
    let part1 = part2.clone().flat_map(|(x, y, z, w)| [(x, y, z, w),(z, w, x, y)].into_iter().unique());
    
    println!("Day 4-1 answer: {}", part1.filter(|(x, y, z, w)| x >= z && y <= w).count());
    println!("Day 4-2 answer: {}", part2.filter(|(x, y, z, w)| y >= z && x <= w).count());
}
