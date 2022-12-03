use itertools::Itertools;

pub fn run()
{
    let input = std::fs::read_to_string("input/day2.txt").unwrap();
    let scores = input.trim_end().split("\n")
        .map(|round| round.split_whitespace()
            .map(|choice| choice.chars().next().unwrap() as i32)
            .next_tuple::<(i32, i32)>().unwrap())
        .map(|(theirs, mine)| {
            let (theirs_idx, mine_idx) = (theirs - 65, mine - 88);
            (((theirs_idx - mine_idx - 3).abs() % 3 + 1) % 3 * 3 + mine_idx + 1,
            (theirs_idx + mine_idx + 2) % 3 + 1 + mine_idx * 3)})
        .collect::<Vec<(i32, i32)>>();

    println!("Day 2-1 answer: {}", scores.iter().map(|(score, _)| score).sum::<i32>());
    println!("Day 2-2 answer: {}", scores.iter().map(|(_, score)| score).sum::<i32>());
}
