pub fn run()
{
    part1();
    part2();
}

fn part1()
{
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let values: Vec<i32> = input.split_whitespace()
       .map(|line| line.parse::<i32>().unwrap())
       .collect();

    let mut prev_value: i32 = i32::MAX;
    let mut increments: i32 = 0;

    for value in values
    {
        if value > prev_value
        {
            increments += 1;
        }

        prev_value = value;
    }

    println!("Day 1-1 answer: {}", increments);
}

fn part2()
{
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let values: Vec<i32> = input.split_whitespace()
       .map(|line| line.parse::<i32>().unwrap())
       .collect();
    
    let mut prev_sum: i32 = i32::MAX;
    let mut increments: i32 = 0;
    
    for window in values.windows(3)
    {
        let sum = window.iter().sum();

        if sum > prev_sum
        {
            increments += 1;
        }

        prev_sum = sum;
    }

    println!("Day 1-2 answer: {}", increments);
}
