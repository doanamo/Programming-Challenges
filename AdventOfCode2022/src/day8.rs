use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

pub fn run()
{
    let mut map = include_str!("../input/day8.txt").lines()
        .map(|line| line.chars()
            .map(|char| (char.to_digit(10).unwrap() as i32, false, 1))
            .collect_vec())
        .collect_vec();

    for _ in 0..4
    {
        map = map.iter()
            .map(|row| row.iter().enumerate()
                .map(|(i, &(height, visible, score))| {
                    (height, visible || height > row.iter().take(i)
                        .map(|&(height,_, _)| height).max().unwrap_or(-1),
                    score * row.iter().take(i).rev()
                        .fold_while(0, |acc, &(other_height, _, _)|
                            if height > other_height { Continue(acc + 1) }
                            else { Done(acc + 1) }).into_inner())})
                .collect_vec())
            .collect_vec();

        map = (0..map[0].len()).rev()
            .map(|i| map.iter()
                .map(|row| row[i])
                .collect_vec())
            .collect_vec();
    }

    let results = map.iter().flatten()
        .filter(|(_, visible,_)| *visible)
        .map(|(_, _, score)| score)
        .collect_vec();
    
    println!("Day 8-1 answer: {}", results.len());
    println!("Day 8-2 answer: {}", results.iter().max().unwrap());
}
