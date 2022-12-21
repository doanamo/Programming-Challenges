use itertools::Itertools;

pub fn run()
{
    let input = include_str!("../input/day6.txt")
        .chars().collect::<Vec<char>>();
    let find_marker = |length| input.windows(length)
        .find_position(|seq| seq.iter().all_unique())
        .unwrap().0 + length;

    println!("Day 6-1 answer: {}", find_marker(4));
    println!("Day 6-2 answer: {}", find_marker(14));
}
