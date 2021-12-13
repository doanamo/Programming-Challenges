
pub fn run()
{
    use std::ops::Index;
    use itertools::Itertools;

    let file = std::fs::read_to_string("input/day13.txt").unwrap();
    let inputs: Vec<&str> = file.split("\n\n").collect();

    let mut dots: Vec<(i32, i32)> = inputs
        .index(0).split("\n")
        .map(|line| line.split(",")
            .map(|value| value.parse().unwrap())
            .collect_tuple().unwrap())
        .collect();

    let folds: Vec<(Option<i32>, Option<i32>)> = inputs
        .index(1).split("\n")
        .map(|line| line.split("=").collect_tuple().unwrap())
        .map(|(fold, units)| match fold {
            "fold along x" => (Some(units.parse().unwrap()), None),
            _ => (None, Some(units.parse().unwrap()))})
        .collect();

    let mut first_fold = None;
    for (fold_x, fold_y) in folds.iter()
    {
        for (dot_x, dot_y) in dots.iter_mut()
        {
            if let Some(fold_x) = fold_x
            {
                *dot_x = 2 * fold_x.min(dot_x) - *dot_x;
            }
            
            if let Some(fold_y) = fold_y
            {
                *dot_y = 2 * fold_y.min(dot_y) - *dot_y;
            }
        }

        dots.sort();
        dots.dedup();
        if let None = first_fold
        {
            first_fold = Some(dots.len());
        }
    }

    println!("Day 13-1 answer: {}", first_fold.unwrap());
    println!("Day 13-2 answer: {}",
    {
        let mut output = String::new();
        for y in 0..=*dots.iter().map(|(_, y)| y).max().unwrap()
        {
            output.push('\n');
            for x in 0..=*dots.iter().map(|(x, _)| x).max().unwrap()
            {
                match dots.iter().find(|&&t| t == (x, y))
                {
                    Some(_) => output.push('#'),
                    None => output.push('.')
                }
            }
        }

        output
    });
}
