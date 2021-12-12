struct Context
{
    pub size_x: i32,
    pub size_y: i32,
    pub heightmap: Vec<u32>,
    pub basins: Vec<bool>,
}

impl Context
{
    fn get_height(&self, x: i32, y: i32) -> Option<&u32>
    {
        if x < 0 || x >= self.size_x || y < 0 || y >= self.size_y
        {
            return None
        }
        
        self.heightmap.get((self.size_x * y + x) as usize)
    }

    fn get_adjacent_height_min(&self, x: i32, y:i32) -> &u32
    {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
            .map(|(x, y)| self.get_height(x, y))
            .filter_map(|o| o).min().unwrap()
    }

    fn recursive_basin_flood(&mut self, x: i32, y: i32) -> u32
    {
        if let Some(height) = self.get_height(x, y).cloned()
        {
            let basin_processed = &mut self.basins[(self.size_x * y + x) as usize];
            if height >= 9 || *basin_processed
            {
                return 0
            }
            
            *basin_processed = true;
            return [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .map(|(x, y)| self.recursive_basin_flood(x, y)).sum::<u32>() + 1
        }

        return 0
    }
}

pub fn run()
{
    let file = std::fs::read_to_string("input/day9.txt").unwrap();
    let size_x = file.find('\n').unwrap() as i32;
    let size_y = file.split('\n').count() as i32;

    let mut context = Context
    {
        size_x, size_y,
        heightmap: file.chars().filter_map(|c| c.to_digit(10)).collect(),
        basins: vec![false; (size_x * size_y) as usize]
    };

    let mut answer_one = 0;
    let mut answer_two: Vec<u32> = Vec::new();
    for y in 0..size_y
    {
        for x in 0..size_x
        {
            let height = context.get_height(x, y).unwrap();
            if height < context.get_adjacent_height_min(x, y)
            {
                answer_one += height + 1;

                let new_basin_size = context.recursive_basin_flood(x, y);
                if new_basin_size != 0
                {
                    answer_two.push(new_basin_size);
                    answer_two.sort_by(|a, b| a.cmp(b).reverse());
                    answer_two.truncate(3);
                }
            }
        }
    }

    println!("Day 9-1 answer: {}", answer_one);
    println!("Day 9-2 answer: {}", answer_two.iter().fold(1, |a, b| a * b));
}
