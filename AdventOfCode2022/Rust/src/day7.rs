use sscanf::sscanf;

pub fn run()
{
    let mut stack = Vec::<usize>::new();
    let mut sizes = Vec::<usize>::new();
    let mut pop = |stack: &mut Vec<usize>|
    {
        let pop = stack.pop().unwrap();
        if let Some(top) = stack.last_mut()
        {
            *top = *top + pop;
        }
        sizes.push(pop);
    };

    for line in include_str!("../input/day7.txt").lines()
    {
        if let Ok(name) = sscanf!(line, "$ cd {str}")
        {
            match name
            {
                ".." => pop(&mut stack),
                _ => stack.push(0)
            }
        }
        else if let Ok((size, _)) = sscanf!(line, "{usize} {str}")
        {
            let top = stack.last_mut().unwrap();
            *top = *top + size;
        }
    }
    
    (0..stack.len()).for_each(|_| pop(&mut stack));
    let free = 70000000 - sizes.last().unwrap();
    sizes.sort();

    println!("Day 7-1 answer: {}", sizes.iter().filter(|&size| *size < 100000).sum::<usize>());
    println!("Day 7-2 answer: {}", sizes.iter().find(|&size| free + *size >= 30000000).unwrap());
}
