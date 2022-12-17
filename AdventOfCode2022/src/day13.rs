use std::cmp::Ordering;
use itertools::Itertools;

pub fn compare(left: &[u8], right: &[u8]) -> Ordering
{
    match (left[0], right[0])
    {
        (l, r) if l == r => compare(&left[1..], &right[1..]),
        (_, b']') => Ordering::Greater,
        (b']', _) => Ordering::Less,
        (b'[', _) => compare(&left[1..], &[&[right[0], b']'], &right[1..]].concat()),
        (_, b'[') => compare(&[&[left[0], b']'], &left[1..]].concat(), &right[1..]),
        (_, _) => left[0].cmp(&right[0]),
    }
}

pub fn run()
{
    let input = include_str!("../input/day13.txt").replace("10", "A");
    let pairs: Vec<(&str, &str)> = input.split("\n\n")
        .map(|pair| pair.split_whitespace().collect_tuple().unwrap())
        .collect_vec();

    let mut part1 = 0;
    for (i, (left, right)) in pairs.iter().enumerate()
    {
        match compare(left.as_bytes(), right.as_bytes())
        {
            Ordering::Less => part1 = part1 + i + 1,
            _ => {},
        }
    }

    let part2 = pairs.into_iter().flat_map(|(left, right)| vec![left, right]).chain(
        vec!["[[2]]", "[[6]]"].into_iter()).sorted_by(|left, right| compare(left.as_bytes(), right.as_bytes()))
        .collect_vec();

    println!("Day 13-1 answer: {}", part1);
    println!("Day 13-2 answer: {}", 
        (part2.iter().position(|&divider| divider == "[[2]]").unwrap() + 1) *
        (part2.iter().position(|&divider| divider == "[[6]]").unwrap() + 1))
}
