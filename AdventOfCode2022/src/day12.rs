use std::collections::HashSet;
use itertools::Itertools;

pub fn run()
{
    let (width, height, mut map) = include_str!("../input/day12.txt").lines()
        .map(|line| line.chars().collect_vec()).fold((0, 0, Vec::<char>::new()),
        |(_, height, map), line| (line.len() as i32, height + 1, [map, line].concat()));

    let start = map.iter().position(|&c| c == 'S').unwrap();
    let end = map.iter().position(|&c| c == 'E').unwrap();
    map[start] = 'a';
    map[end] = 'z';

    let mut distances = vec![usize::MAX; map.len()];
    let mut previous = vec![usize::MAX; map.len()];
    let mut visited = vec![false; map.len()];
    let mut queue = HashSet::from([end]);
    distances[end] = 0;
    
    while !queue.is_empty()
    {
        let (unvisited_index, unvisited_distance) = queue.iter()
            .map(|&unvisited| (unvisited, distances[unvisited]))
            .min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        visited[unvisited_index] = true;
        queue.remove(&unvisited_index);

        for neighbor in [(1, 0), (-1, 0), (0, 1), (0, -1)]
        {
            let current_x = unvisited_index as i32 % width;
            let current_y = unvisited_index as i32 / width;
            let neighbor_x = current_x + neighbor.0;
            let neighbor_y = current_y + neighbor.1;
            if neighbor_x >= 0 && neighbor_x < width && neighbor_y >= 0 && neighbor_y < height
            {
                let neighbor_index = (neighbor_y * width + neighbor_x) as usize;
                if !visited[neighbor_index]
                {
                    let neighbor_height = map[neighbor_index] as i32;
                    let unvisited_height = map[unvisited_index] as i32;
                    if (unvisited_height - neighbor_height) <= 1
                    {
                        let distance = unvisited_distance + 1;
                        if distance < distances[neighbor_index]
                        {
                            distances[neighbor_index] = distance;
                            previous[neighbor_index] = unvisited_index;
                        }
                        queue.insert(neighbor_index);
                    }
                }
            }
        }
    }

    println!("Day 12-1 answer: {}", distances[start]);
    println!("Day 12-2 answer: {}", map.iter().enumerate().filter(
        |(_, &c)| c == 'a').map(|(i, _)| distances[i]).min().unwrap())
}