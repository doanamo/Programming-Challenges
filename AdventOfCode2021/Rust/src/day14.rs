
pub fn run()
{
    use std::ops::Index;
    use std::collections::HashMap;
    use itertools::Itertools;

    let inputs: Vec<String> = std::fs::read_to_string("input/day14.txt").unwrap()
        .split("\n\n").map(|s| s.to_string()).collect();

    let template: Vec<char> = inputs.index(0).chars().collect_vec();
    let insertions: HashMap<(char, char), char> = inputs.index(1).split("\n")
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(pair, insert)| (
            pair.chars().collect_tuple().unwrap(),
            insert.chars().next().unwrap()))
        .collect();

    let polymerization = |iterations: usize|
    {
        let mut pair_count = template.windows(2)
            .fold(HashMap::<(char, char), usize>::new(), |mut map, pair| {
                *map.entry((pair[0], pair[1])).or_insert(0) += 1; map});

        let mut element_count = template.iter()
            .fold(HashMap::<char, usize>::new(), |mut map, &element| {
                *map.entry(element).or_insert(0) += 1; map});

        for _ in 0..iterations
        {
            let mut new_pairs = pair_count.clone();
            for (&(first, second), &count) in pair_count.iter()
            {
                if let Some(&insert) = insertions.get(&(first, second))
                {
                    *new_pairs.get_mut(&(first, second)).unwrap() -= count;
                    *new_pairs.entry((first, insert)).or_insert(0) += count;
                    *new_pairs.entry((insert, second)).or_insert(0) += count;
                    *element_count.entry(insert).or_insert(0) += count;
                }
            }
            pair_count = new_pairs;
        }

        let ((_, min), (_, max)) = element_count.into_iter()
            .minmax_by_key(|(_, v)| *v).into_option().unwrap();
        
        max - min
    };

    println!("Day 14-1 answer: {}", polymerization(10));
    println!("Day 14-2 answer: {}", polymerization(40));
}
