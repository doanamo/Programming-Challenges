use std::cell::RefCell;
use itertools::Itertools;
use sscanf::sscanf;

struct Monkey
{
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    branch: (u64, usize, usize),
    inspections: u64,
}

fn chase_monkeys(rounds: u64, worry_reduction: u64) -> u64
{
    let monkeys = include_str!("../input/day11.txt").split("\n\n")
        .map(|monkey| monkey.lines().map(|line| line.trim()).collect_vec()).map(|lines| {
            let (items, op, divisor, yes, no) = (
                sscanf!(lines[1], "Starting items: {str}").unwrap(),
                sscanf!(lines[2], "Operation: new = old {str}").unwrap(),
                sscanf!(lines[3], "Test: divisible by {u64}").unwrap(),
                sscanf!(lines[4], "If true: throw to monkey {usize}").unwrap(),
                sscanf!(lines[5], "If false: throw to monkey {usize}").unwrap());
            RefCell::new(Monkey {
                items: items.split(", ").map(|item|
                    item.parse::<u64>().unwrap()).collect_vec(),
                op: match sscanf!(op, "{str} {str}").unwrap() {
                    ("+", "old") => Box::new(|old| old + old),
                    ("*", "old") => Box::new(|old| old * old),
                    (sign, operand) => {
                        match (sign, operand.parse::<u64>().unwrap()) {
                            ("+", value) => Box::new(move |old| old + value),
                            ("*", value) => Box::new(move |old| old * value),
                            _ => panic!()}}},
                branch: (divisor, yes, no),
                inspections: 0})})
        .collect_vec();

    let common_divisor = monkeys.iter().map(|monkey| monkey.borrow().branch.0)
        .fold(1, |acc, divisor| acc * divisor);

    for _ in 0..rounds
    {
        for i in 0..monkeys.len()
        {
            let mut monkey = monkeys[i].borrow_mut();
            for item in monkey.items.clone()
            {
                let worry = (monkey.op)(item % common_divisor) / worry_reduction;
                let mut target = monkey.branch.2;
                if worry % monkey.branch.0 == 0
                {
                    target = monkey.branch.1
                }
                monkeys[target].borrow_mut().items.push(worry);
                monkey.inspections = monkey.inspections + 1;
            }
            monkey.items.clear();
        }
    }

    monkeys.iter().map(|monkey| monkey.borrow().inspections)
        .sorted().rev().take(2).fold(1, |acc, value| acc * value)
}

pub fn run()
{
    println!("Day 11-1 answer: {}", chase_monkeys(20, 3));
    println!("Day 11-2 answer: {}", chase_monkeys(10000, 1));
}