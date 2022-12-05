use itertools::Itertools;

pub fn run()
{
    let input = include_str!("../input/day3.txt");
    let part1 = input.lines()
        .map(|line| line.split_at(line.chars().count() / 2))
        .map(|(left, right)| left.chars()
            .filter(|char| right.contains(*char))
            .next().unwrap() as u32);
    let part2 = input.lines().tuples::<(_, _, _)>()
        .map(|(first, second, third)| first.chars()
            .filter(|char| second.contains(*char))
            .filter(|char| third.contains(*char))
            .next().unwrap() as u32);
    let scoring = |score, char: u32|
        score + char - 38 - char / 96 * 58;

    println!("Day 3-1 answer: {}", part1.fold(0, scoring));
    println!("Day 3-2 answer: {}", part2.fold(0, scoring));
}
