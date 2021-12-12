use std::{collections::HashMap, ops::Index};
use itertools::Itertools;

pub fn run()
{
    let file = std::fs::read_to_string("input/day10.txt").unwrap();

    let closing_pairs: HashMap<char, char> = HashMap::from(
        [(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut error_score = 0;
    let error_scoring: HashMap<char, u32> = HashMap::from([
        (')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut completion_scores: Vec<u64> = Vec::new();
    let completion_scoring: HashMap<char, u64> = HashMap::from([
        ('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    for line in file.split("\n")
    {
        let mut syntax_error = false;
        let mut stack: Vec<char> = Vec::new();
        for character in line.chars()
        {
            if ['(', '[', '{', '<'].contains(&character)
            {
                stack.push(character);
            }
            else if stack.last().unwrap() == &closing_pairs[&character]
            {
                stack.pop();
            }
            else
            {
                syntax_error = true;
                error_score += error_scoring[&character];
                break
            }
        }

        if !syntax_error
        {
            completion_scores.push(stack.iter().rev()
                .fold(0, |a, b| a * 5 + completion_scoring[b]));
        }
    }

    println!("Day 10-1 answer: {}", error_score);
    println!("Day 10-2 answer: {}", completion_scores.iter()
        .sorted().collect::<Vec<&u64>>().index(completion_scores.len() / 2));
}
