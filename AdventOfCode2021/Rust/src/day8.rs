use itertools::Itertools;
use std::collections::HashMap;

pub fn run()
{
    let file = std::fs::read_to_string("input/day8.txt").unwrap();
    let pairs: Vec<(Vec<String>, Vec<String>)> = file
        .split('\n')
        .map(|line| line
            .split(" ")
            .map(|set| set.chars().sorted().collect::<String>())
            .join(" "))
        .map(|line| line
            .splitn(2, " | ")
            .map(|s| s
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .collect_tuple()
            .unwrap())
        .collect();

    let mut found = 0;
    let mut sum = 0;

    for pair in pairs
    {
        let (inputs, outputs) = &mut pair.clone();
        fn extract_digit(inputs: &mut Vec<String>, digits: &Vec<String>,
            length: usize, comparee: usize, matches: usize) -> String
        {
            let index = inputs.iter().position(|s| s.len() == length &&
                s.chars().filter(|c| digits[comparee].contains(*c)).count() == matches);
            let result = inputs[index.unwrap()].clone();
            inputs.remove(index.unwrap());
            result
        }

        let mut digits: Vec<String> = vec![String::new(); 10];
        digits[1] = extract_digit(inputs, &digits, 2, 0, 0);
        digits[4] = extract_digit(inputs, &digits, 4, 0, 0);
        digits[7] = extract_digit(inputs, &digits, 3, 0, 0);
        digits[8] = extract_digit(inputs, &digits, 7, 0, 0);
        digits[3] = extract_digit(inputs, &digits, 5, 1, 2);
        digits[2] = extract_digit(inputs, &digits, 5, 4, 2);
        digits[5] = extract_digit(inputs, &digits, 5, 0, 0);
        digits[6] = extract_digit(inputs, &digits, 6, 1, 1);
        digits[9] = extract_digit(inputs, &digits, 6, 3, 5);
        digits[0] = extract_digit(inputs, &digits, 6, 8, 6);

        let digits_map: HashMap<&String, i32> = HashMap::from_iter(digits.iter()
            .enumerate().map(|(i, digit)| (digit, i as i32)));

        for (i, output) in outputs.iter().enumerate()
        {
            if [1, 4, 7, 8].contains(digits_map.get(output).unwrap())
            {
                found += 1;
            }

            sum += digits_map.get(output).unwrap() * 10_i32.pow(3 - i as u32);
        }
    }
    
    println!("Day 8-1 answer: {}", found);
    println!("Day 8-2 answer: {}", sum);
}
