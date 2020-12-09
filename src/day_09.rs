use crate::utils::get_file;
use itertools::{min, max};


const PREAMBLE_LENGTH: usize = 25;


pub fn day_09() {
    let input = get_input();

    let solution_a = solve_part_a(&input);
    println!("Part A: {}", solution_a);

    let solution_b = solve_part_b(&input, solution_a);
    println!("Part B: {}", solution_b);
}


fn get_input() -> Vec<u64> {
    get_file("./inputs/day_09.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap()).collect()
}


pub fn solve_part_a(numbers: &[u64]) -> u64 {
    let idx_found = (0..(numbers.len() - PREAMBLE_LENGTH))
        .find(|&idx|
            !is_sum_of_two(&numbers[idx..(PREAMBLE_LENGTH + idx)], numbers[PREAMBLE_LENGTH + idx])
        ).unwrap();
    numbers[idx_found + PREAMBLE_LENGTH]
}


fn is_sum_of_two(numbers: &[u64], number: u64) -> bool {
    numbers.iter()
        .any(|&n| number > n && numbers.contains(&(number - n)))
}


pub fn solve_part_b(numbers: &[u64], number: u64) -> u64 {
    for end in 0..numbers.len() {
        for start in 0..end {
            let slice = &numbers[start..end];
            if slice.iter().sum::<u64>() == number {
                return min(slice.iter()).unwrap() + max(slice.iter()).unwrap();
            }
        }
    }
    unreachable!()
}
