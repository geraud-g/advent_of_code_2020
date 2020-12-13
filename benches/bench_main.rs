use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2020::day_09::{solve_part_a, solve_part_b};
use aoc_2020::utils::get_file;
use aoc_2020::day_04::day_04;
use aoc_2020::day_03::day_03;
use aoc_2020::day_11::day_11;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "test_day_03",
        |b| b.iter(|| day_03()));

    c.bench_function(
        "test_day_04",
        |b| b.iter(|| day_04()));

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
    c.bench_function(
        "test_day_11",
        |b| b.iter(|| day_11()));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
