use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2020::day_09::{solve_part_a, solve_part_b};
use aoc_2020::utils::get_file;


fn criterion_benchmark(c: &mut Criterion) {
    let input: Vec<u64> = get_file("./inputs/day_09.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap()).collect();

    c.bench_function(
        "test_day_09_part_A",
        |b| b.iter(||
            solve_part_a(black_box(&input))
        ));

    let result_part_a = solve_part_a(&input);
    c.bench_function(
        "test_day_09_part_B",
        |b| b.iter(||
            solve_part_b(black_box(&input), result_part_a)
        ));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
