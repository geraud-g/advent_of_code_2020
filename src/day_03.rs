use crate::utils::get_file;


const TREE: char = '#';


pub fn day_03() {
    let data = get_input();

    let solution_a = traverse_map(&data, 1, 3);
    println!("Part A: {}", solution_a);

    let solution_b = solve_part_b(&data);
    println!("Part B: {}", solution_b);
}


fn get_input() -> Vec<Vec<char>> {
    let data = get_file("./inputs/day_03.txt");
    data.lines().map(|l| l.chars().collect()).collect()
}


fn traverse_map(area: &[Vec<char>], delta_y: usize, delta_x: usize) -> u64 {
    let x_len = area[0].len();
    let mut x = 0;
    let mut tree_count = 0;

    for y in (0..area.len()).step_by(delta_y) {
        let tile = area[y].get(x).unwrap();
        if tile.eq(&TREE) {
            tree_count += 1
        }
        x = (x + delta_x) % x_len;
    }

    tree_count
}


fn solve_part_b(area: &[Vec<char>]) -> u64 {
    traverse_map(&area, 1, 1) *
        traverse_map(&area, 1, 3) *
        traverse_map(&area, 1, 5) *
        traverse_map(&area, 1, 7) *
        traverse_map(&area, 2, 1)
}
