use itertools::Itertools;

pub fn run()
{
    let (mut width, mut depth) = (0, 0);
    let walls = include_str!("../input/day14.txt").lines()
        .flat_map(|line| line.split(" -> ")
            .map(|wall| wall.split(",").collect_tuple::<(_, _)>().unwrap())
                .map(|(str_x, str_y)| {
                    let x = str_x.parse::<i32>().unwrap();
                    let y = str_y.parse::<i32>().unwrap();
                    width = std::cmp::max(width, x * 2);
                    depth = std::cmp::max(depth, y + 2);
                    (x, y)})
            .tuple_windows::<(_, _)>()
            .collect_vec())
        .collect_vec();
    
    let mut map = vec!['.'; (width * depth) as usize];
    map.append(&mut vec!['#'; width as usize]);
    for (begin, end) in walls
    {
        let vector = (end.0 - begin.0, end.1 - begin.1); 
        let segments = std::cmp::max(vector.0.abs(), vector.1.abs());
        for i in 0..=segments
        {
            let alpha = i as f32 / segments as f32;
            let x = begin.0 + (vector.0 as f32 * alpha) as i32;
            let y = begin.1 + (vector.1 as f32 * alpha) as i32;
            map[(y * width + x) as usize] = '#';
        }
    }

    let (mut part1, mut part2) = (None, None);
    for i in 0..
    {
        if map[500] == 'o'
        {
            part2 = Some(i);
            break;
        }
        
        let mut sand = (500, 0); 
        'sand: loop
        {
            if part1 == None && sand.1 == depth - 2
            {
                part1 = Some(i);
            }

            for offset in [(0, 1), (-1, 1), (1, 1)]
            {
                let position = (sand.0 + offset.0, sand.1 + offset.1);
                if map[(position.1 * width + position.0) as usize] == '.'
                {
                    sand = position;
                    continue 'sand;
                }
            }

            map[(sand.1 * width + sand.0) as usize] = 'o';
            break;
        }
    }

    println!("Day 14-1 answer: {}", part1.unwrap());
    println!("Day 14-2 answer: {}", part2.unwrap());
}