use itertools::Itertools;

pub fn run()
{
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    let positions: Vec<i32> = input
        .split(',')
        .map(|p| p.parse::<i32>().unwrap())
        .sorted()
        .collect();

    print!("Day 7-1 answer: {}\n", 
    {
        let median = positions[positions.len() / 2];
        positions.iter()
            .map(|p| (p - median).abs())
            .sum::<i32>()
    });

    print!("Day 7-2 answer: {}\n", 
    {
        fn fuel_formula(x: i32) -> i32 { x * (x + 1) / 2 }
        let (&min, &max) = positions.iter()
            .minmax().into_option().unwrap();

        (min..=max).into_iter()
            .map(|p1| positions.iter()
                .map(|p2| fuel_formula((p2 - p1).abs()))
                .sum::<i32>())
            .min().unwrap()
    });
}
