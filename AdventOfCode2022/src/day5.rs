use sscanf::sscanf;

pub fn run()
{
    let (stacks, moves) = include_str!("../input/day5.txt")
        .split_once("\n\n").unwrap();
    
    let mut part1 = vec![Vec::<char>::new(); 9];
    for row in stacks.lines().rev().skip(1)
    {
        for (chunk, stack) in row.as_bytes().chunks(4).zip(part1.iter_mut())
        {
            let string = std::str::from_utf8(chunk).unwrap();
            if let Ok(cargo) = sscanf!(string.trim(), "[{char}]")
            {
                stack.push(cargo);
            }
        }
    }

    let mut part2 = part1.clone();
    for line in moves.lines()
    {
        let (count, from, to) = sscanf!(line, "move {u32} from {usize} to {usize}").unwrap();
        let mut ordered = Vec::<char>::new();
        for _ in 0..count
        {
            if let Some(cargo) = part1.get_mut(from - 1).unwrap().pop()
            {
                part1.get_mut(to - 1).unwrap().push(cargo);
            }
            
            if let Some(cargo) = part2.get_mut(from - 1).unwrap().pop()
            {
                ordered.insert(0, cargo);
            }
        }
        part2.get_mut(to - 1).unwrap().append(&mut ordered);
    }

    println!("Day 5-1 answer: {}", part1.iter().filter_map(|stack| stack.last()).collect::<String>());
    println!("Day 5-2 answer: {}", part2.iter().filter_map(|stack| stack.last()).collect::<String>());
}
