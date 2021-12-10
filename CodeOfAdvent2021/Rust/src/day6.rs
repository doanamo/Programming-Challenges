pub fn run()
{
    part1();
    part2();
}

fn part1()
{
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    let mut fishes: Vec<i32> = input
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect();

    for _ in 1..=80
    {
        let mut spawned: Vec<i32> = Vec::new();
        for fish in &mut fishes
        {
            *fish -= 1;

            if *fish < 0
            {
                *fish = 6;
                spawned.push(8);
            }
        }
        fishes.append(&mut spawned);
    }

    print!("Day 6-1 answer: {}\n", fishes.len());
}

fn part2()
{
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    let initial_lifetimes: Vec<i32> = input
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect();

    let mut lifetimes: Vec<i64> = Vec::new();
    lifetimes.resize(9, 0);

    for lifetime in initial_lifetimes
    {
        lifetimes[lifetime as usize] += 1
    }

    for _ in 1..=256
    {
        let mut spawns: i64 = 0;
        let mut new_lifetimes: Vec<i64> = Vec::new();
        new_lifetimes.resize(9, 0);

        for (i, fishes) in lifetimes.iter().enumerate()
        {
            if i == 0
            {
                spawns = *fishes
            }
            else
            {
                new_lifetimes[i - 1] = *fishes;
            }
        }

        new_lifetimes[6] += spawns;
        new_lifetimes[8] += spawns;
        lifetimes = new_lifetimes;
    }

    print!("Day 6-2 answer: {}\n", lifetimes.iter().sum::<i64>());
}
