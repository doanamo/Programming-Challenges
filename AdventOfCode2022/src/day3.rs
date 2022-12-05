use itertools::Itertools;

pub fn run()
{
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    let part1 = input.lines()
        .map(|line| line.split_at(line.chars().count() / 2))
        .map(|(left, right)| left.chars()
            .filter(|char| right.find(*char).is_some())
            .next().unwrap() as u32);
    let part2 = input.lines().tuples::<(_, _, _)>()
        .map(|(first, second, third)| first.chars()
            .filter(|char| second.find(*char).is_some())
            .filter(|char| third.find(*char).is_some())
            .next().unwrap() as u32);
    let scoring = |score, char: u32|
        if char > 96 { score + char - 96 }
        else { score + char - 38 };

    println!("Day 3-1 answer: {}", part1.fold(0, scoring));
    println!("Day 3-2 answer: {}", part2.fold(0, scoring));
}
