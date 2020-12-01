use std::fs;
use itertools::Itertools;


pub fn day_01() {
    let input = get_input();

    println!("Day 01 Part A: {}", solve_part_a(&input));

    println!("Day 01 Part B: {}", solve_part_b(&input));
}


fn get_input() -> Vec<i32> {
    let data = fs::read_to_string("./inputs/day_01.txt")
        .expect("Unable to read file");
    data.lines().map(|s| s.parse().unwrap()).collect()
}


fn solve_part_a(input: &Vec<i32>) -> i32 {
    solve(input, 2).expect("No solution found for Day 01 - Part A")
}


fn solve_part_b(input: &Vec<i32>) -> i32 {
    solve(input, 3).expect("No solution found for Day 01 - Part B")
}


fn solve(input: &Vec<i32>, entries_nbr: usize) -> Option<i32> {
    let valid_combinations = input.iter()
        .filter(|i| **i < 2020)
        .cloned()
        .combinations(entries_nbr).
        find(|comb| comb.iter().sum::<i32>() == 2020);

    match valid_combinations {
        Some(value) => { Some(value.iter().product()) }
        _ => None
    }
}
