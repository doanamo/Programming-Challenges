pub fn run()
{
    part1();
    part2();
}

fn part1()
{
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    let values: Vec<u32> = input
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    let bits = 12;
    for bit in 0..bits
    {
        let mut ones: usize = 0;
        for value in &values
        {
            if value & (1 << bit) != 0
            {
                ones += 1
            }
        }

        if ones >= values.len() / 2
        {
            gamma += 1 << bit
        }
        else
        {
            epsilon += 1 << bit
        }
    }

    print!("Day 3-1 answer: {}\n", epsilon * gamma)
}

fn part2()
{
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    let values: Vec<&str> = input
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut generator_values = values.clone();
    let mut scrubber_values = values.clone();

    let mut generator: Option<u32> = None;
    let mut scrubber: Option<u32> = None;

    let bits = 12;
    for bit in 0..bits
    {
        if generator == None
        {
            let mut ones: usize = 0;
            for value in &generator_values
            {
                if value.chars().nth(bit) == Some('1')
                {
                    ones += 1
                }
            }

            let mut expected = '0';
            if ones >= generator_values.len() - ones
            {
                expected = '1'
            }

            generator_values = generator_values.iter()
                .filter(|&&x| x.chars().nth(bit) == Some(expected))
                .cloned()
                .collect();
            
            if generator_values.len() == 1
            {
                generator = Some(u32::from_str_radix(generator_values.first().cloned().unwrap(), 2).unwrap())
            }
        }

        if scrubber == None
        {
            let mut ones: usize = 0;
            for value in &scrubber_values
            {
                if value.chars().nth(bit) == Some('1')
                {
                    ones += 1
                }
            }

            let mut expected = '0';
            if ones < scrubber_values.len() - ones
            {
                expected = '1'
            }

            scrubber_values = scrubber_values.iter()
                .filter(|&&x| x.chars().nth(bit) == Some(expected))
                .cloned()
                .collect();

            if scrubber_values.len() == 1
            {
                scrubber = Some(u32::from_str_radix(scrubber_values.first().cloned().unwrap(), 2).unwrap())
            }
        }
    }

    print!("Day 3-2 answer: {}\n", generator.unwrap() * scrubber.unwrap())
}
