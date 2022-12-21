use std::collections::HashSet;
use sscanf::sscanf;

pub fn run()
{
    let moves: Vec<(i32, i32)> = include_str!("../input/day9.txt").lines()
        .flat_map(|line| match sscanf!(line, "{char} {usize}").unwrap() {
            ('R', i) => vec![(1, 0); i], ('L', i) => vec![(-1, 0); i],
            ('U', i) => vec![(0, 1); i], ('D', i) => vec![(0, -1); i],
            _ => panic!()})
        .collect();
    
    let simulate = |length|
    {
        let mut visited = HashSet::new();
        let mut rope = vec![(0, 0); length];
        for offset in moves.iter()
        {
            rope[0] = (rope[0].0 + offset.0, rope[0].1 + offset.1);
            for i in 0..(rope.len() - 1)
            {
                let (front, current) = (rope[i], &mut rope[i + 1]);
                let offset = (front.0 - current.0, front.1 - current.1);
                if offset.0.abs() > 1 || offset.1.abs() > 1
                {
                    current.0 = current.0 + offset.0.signum();
                    current.1 = current.1 + offset.1.signum();
                }
            }
            visited.insert(*rope.last().unwrap());
        }
        visited
    };

    println!("Day 9-1 answer: {}", simulate(2).len());
    println!("Day 9-2 answer: {}", simulate(10).len());
}
