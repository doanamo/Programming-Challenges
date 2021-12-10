use std::str::FromStr; 

pub fn run()
{
    part1();
    part2();
}

struct Command
{
    direction: String,
    units: i32,
}

impl FromStr for Command
{
    type Err = std::num::ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        let tokens: Vec<&str> = string.split(' ').collect();
        assert!(tokens.len() == 2);

        Ok(Command
        {
            direction: String::from(tokens[0]),
            units: tokens[1].parse::<i32>().unwrap(),
        })
    }
}

fn part1()
{
    let input = std::fs::read_to_string("input/day2.txt").unwrap();
    let commands: Vec<Command> = input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| Command::from_str(line).unwrap())
        .collect();

    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands
    {
        match command.direction.as_str()
        {
            "forward" => horizontal += command.units,
            "down" => depth += command.units,
            "up" => depth -= command.units,
            _ => panic!("Unexpected direction!")
        }
    }

    println!("Day 2-1 answer: {}", horizontal * depth);
}

fn part2()
{
    let input = std::fs::read_to_string("input/day2.txt").unwrap();
    let commands: Vec<Command> = input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| Command::from_str(line).unwrap())
        .collect();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands
    {
        match command.direction.as_str()
        {
            "forward" =>
            {
                horizontal += command.units;
                depth += aim * command.units;
            },
            "down" => aim += command.units,
            "up" => aim -= command.units,
            _ => panic!("Unexpected direction!")
        }
    }

    println!("Day 2-2 answer: {}", horizontal * depth);
}