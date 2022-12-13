use itertools::Itertools;
use sscanf::sscanf;

pub fn run()
{
    let mut ops = include_str!("../input/day10.txt").lines()
        .map(|line| match sscanf!(line, "addx {i32}") {
            Ok(arg) => (2, arg), _ => (1, 0) });

    let (mut register, mut cycle, mut delay, mut add) = (1, 0, 0, 0);
    let mut crt = vec!['.'; 40 * 6];
    let mut compute = |cycles|
    {
        for _ in 0..cycles
        {
            if delay == 0
            {
                register = register + add;
                (delay, add) = ops.next().unwrap();
            }
            if (cycle % 40 - register as i32).abs() <= 1
            {
                crt[cycle as usize] = '#';
            }
            (cycle, delay) = (cycle + 1, delay - 1);
        }
        cycle * register
    };

    println!("Day 10-1 answer: {}", (0..6).map(|i: i32| compute(20 + i.signum() * 20)).sum::<i32>());
    println!("Day 10-2 answer: \n{}", { compute(20); crt.chunks(40).map(|s| s.iter().collect::<String>()).join("\n") });
}